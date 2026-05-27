package com.nicog.inventoryservice.service;

import com.nicog.inventoryservice.entity.Event;
import com.nicog.inventoryservice.entity.Venue;
import com.nicog.inventoryservice.repository.FirebaseInventoryRepository;
import com.nicog.inventoryservice.request.CreateEventRequest;
import com.nicog.inventoryservice.request.CreateVenueRequest;
import com.nicog.inventoryservice.request.UpdateVenueRequest;
import com.nicog.inventoryservice.response.EventInventoryResponse;
import com.nicog.inventoryservice.response.LostUpdateSimulationResponse;
import com.nicog.inventoryservice.response.VenueInventoryResponse;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class InventoryService {

    private final FirebaseInventoryRepository firebaseInventoryRepository;

    public InventoryService(
        final FirebaseInventoryRepository firebaseInventoryRepository
    ) {
        this.firebaseInventoryRepository = firebaseInventoryRepository;
    }

    public List<EventInventoryResponse> getAllEvents() {
        try {
            List<Event> events = firebaseInventoryRepository
                .findAllEvents()
                .join();

            List<EventInventoryResponse> response = new ArrayList<>();

            for (Event event : events) {
                if (event.getLeftCapacity() == null) {
                    log.warn(
                        "Evento con leftCapacity null. eventId={}, eventName={}",
                        event.getId(),
                        event.getName()
                    );
                    continue;
                }

                Venue venue = firebaseInventoryRepository
                    .findVenueById(event.getVenueId())
                    .join();

                response.add(
                    EventInventoryResponse.builder()
                        .eventId(event.getId())
                        .event(event.getName())
                        .capacity(event.getLeftCapacity())
                        .venue(venue.getName())
                        .ticketPrice(event.getTicketPrice())
                        .build()
                );
            }

            return response;
        } catch (Exception exception) {
            log.error("Error obteniendo eventos", exception);
            throw new RuntimeException("No fue posible consultar los eventos");
        }
    }

    public VenueInventoryResponse getVenueInformation(final Long venueId) {
        try {
            Venue venue = firebaseInventoryRepository
                .findVenueById(venueId)
                .join();

            return VenueInventoryResponse.builder()
                .venueId(venue.getId())
                .venueName(venue.getName())
                .address(venue.getAddress())
                .totalCapacity(venue.getTotalCapacity())
                .build();
        } catch (Exception exception) {
            log.error("Error obteniendo sede con id: {}", venueId, exception);
            throw new RuntimeException("No fue posible consultar la sede");
        }
    }

    public EventInventoryResponse getEventInventory(final Long eventId) {
        try {
            Event event = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            Venue venue = firebaseInventoryRepository
                .findVenueById(event.getVenueId())
                .join();

            return EventInventoryResponse.builder()
                .eventId(event.getId())
                .event(event.getName())
                .capacity(event.getLeftCapacity())
                .venue(venue.getName())
                .ticketPrice(event.getTicketPrice())
                .build();
        } catch (Exception exception) {
            log.error(
                "Error obteniendo inventario del evento: {}",
                eventId,
                exception
            );
            throw new RuntimeException(
                "No fue posible consultar el inventario del evento"
            );
        }
    }

    public void updateEventCapacity(
        final Long eventId,
        final Long ticketsBooked
    ) {
        if (ticketsBooked == null || ticketsBooked <= 0) {
            throw new IllegalArgumentException(
                "La cantidad de tickets debe ser mayor a cero"
            );
        }

        try {
            Event eventBefore = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            log.info(
                "Trying to decrease capacity. eventId={}, ticketsBooked={}, currentLeftCapacity={}, totalCapacity={}",
                eventId,
                ticketsBooked,
                eventBefore.getLeftCapacity(),
                eventBefore.getTotalCapacity()
            );

            firebaseInventoryRepository
                .decreaseEventCapacity(eventId, ticketsBooked)
                .join();

            Event eventAfter = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            log.info(
                "Capacity updated. eventId={}, previousLeftCapacity={}, newLeftCapacity={}",
                eventId,
                eventBefore.getLeftCapacity(),
                eventAfter.getLeftCapacity()
            );
        } catch (Exception exception) {
            log.error(
                "Error actualizando capacidad del evento: {} con tickets: {}",
                eventId,
                ticketsBooked,
                exception
            );

            throw new RuntimeException(
                "No fue posible actualizar la capacidad del evento: " +
                    exception.getMessage(),
                exception
            );
        }
    }

    public void releaseEventCapacity(
        final Long eventId,
        final Long ticketsReleased
    ) {
        if (ticketsReleased == null || ticketsReleased <= 0) {
            throw new IllegalArgumentException(
                "La cantidad de tickets liberados debe ser mayor a cero"
            );
        }

        try {
            firebaseInventoryRepository
                .increaseEventCapacity(eventId, ticketsReleased)
                .join();

            log.info(
                "Released event capacity for event id: {} with tickets released: {}",
                eventId,
                ticketsReleased
            );
        } catch (Exception exception) {
            log.error(
                "Error liberando capacidad del evento: {} con tickets: {}",
                eventId,
                ticketsReleased,
                exception
            );

            throw new RuntimeException(
                "No fue posible liberar la capacidad del evento"
            );
        }
    }

    public LostUpdateSimulationResponse simulateLostUpdate(final Long eventId) {
        final Long ticketsToBook = 1L;

        try {
            Event eventBefore = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            Long initialCapacity = eventBefore.getLeftCapacity();

            if (initialCapacity == null) {
                throw new RuntimeException(
                    "El evento no tiene capacidad disponible registrada"
                );
            }

            CompletableFuture<String> requestA = CompletableFuture.supplyAsync(
                () -> {
                    try {
                        Event eventReadByA = firebaseInventoryRepository
                            .findEventById(eventId)
                            .join();

                        Long capacityReadByA = eventReadByA.getLeftCapacity();

                        Thread.sleep(1000);

                        Long newCapacityA = capacityReadByA - ticketsToBook;

                        firebaseInventoryRepository
                            .unsafeSetLeftCapacity(eventId, newCapacityA)
                            .join();

                        return (
                            "SUCCESS: A leyó " +
                            capacityReadByA +
                            " y guardó " +
                            newCapacityA
                        );
                    } catch (Exception exception) {
                        return "FAILED: " + exception.getMessage();
                    }
                }
            );

            CompletableFuture<String> requestB = CompletableFuture.supplyAsync(
                () -> {
                    try {
                        Event eventReadByB = firebaseInventoryRepository
                            .findEventById(eventId)
                            .join();

                        Long capacityReadByB = eventReadByB.getLeftCapacity();

                        Thread.sleep(1000);

                        Long newCapacityB = capacityReadByB - ticketsToBook;

                        firebaseInventoryRepository
                            .unsafeSetLeftCapacity(eventId, newCapacityB)
                            .join();

                        return (
                            "SUCCESS: B leyó " +
                            capacityReadByB +
                            " y guardó " +
                            newCapacityB
                        );
                    } catch (Exception exception) {
                        return "FAILED: " + exception.getMessage();
                    }
                }
            );

            CompletableFuture.allOf(requestA, requestB).join();

            Event eventAfter = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            Long finalCapacity = eventAfter.getLeftCapacity();

            firebaseInventoryRepository
                .unsafeSetLeftCapacity(eventId, finalCapacity)
                .join();

            Long expectedCapacity = initialCapacity - 2;

            boolean lostUpdateOccurred = !expectedCapacity.equals(
                finalCapacity
            );

            return LostUpdateSimulationResponse.builder()
                .eventId(eventId)
                .initialCapacity(initialCapacity)
                .requestACalculatedCapacity(initialCapacity - ticketsToBook)
                .requestBCalculatedCapacity(initialCapacity - ticketsToBook)
                .finalCapacity(finalCapacity)
                .expectedCapacity(expectedCapacity)
                .lostUpdateOccurred(lostUpdateOccurred)
                .requestAStatus(requestA.join())
                .requestBStatus(requestB.join())
                .explanation(
                    "Dos reservas concurrentes leyeron la misma capacidad inicial. " +
                        "Ambas calcularon la nueva capacidad usando ese mismo valor y luego escribieron el resultado. " +
                        "Como no se usó transacción, una actualización sobrescribió a la otra."
                )
                .control(
                    "El problema se controla usando transacciones en Firebase, como en decreaseEventCapacity, " +
                        "donde la lectura y escritura se ejecutan como una operación atómica."
                )
                .build();
        } catch (Exception exception) {
            throw new RuntimeException(
                "No fue posible simular Lost Update",
                exception
            );
        }
    }

    public EventInventoryResponse createEvent(
        final CreateEventRequest request
    ) {
        validateCreateEventRequest(request);

        try {
            Venue venue = firebaseInventoryRepository
                .findVenueById(request.getVenueId())
                .join();

            validateEventCapacityAgainstVenue(request, venue);

            Event event = Event.builder()
                .id(System.currentTimeMillis())
                .name(request.getName())
                .totalCapacity(request.getTotalCapacity())
                .leftCapacity(request.getTotalCapacity())
                .venueId(request.getVenueId())
                .ticketPrice(request.getTicketPrice())
                .build();

            Event savedEvent = firebaseInventoryRepository
                .saveEvent(event)
                .join();

            return EventInventoryResponse.builder()
                .eventId(savedEvent.getId())
                .event(savedEvent.getName())
                .capacity(savedEvent.getLeftCapacity())
                .venue(venue.getName())
                .ticketPrice(savedEvent.getTicketPrice())
                .build();
        } catch (IllegalArgumentException exception) {
            throw exception;
        } catch (Exception exception) {
            log.error("Error creando evento", exception);
            throw new RuntimeException("No fue posible crear el evento");
        }
    }

    private void validateCreateEventRequest(final CreateEventRequest request) {
        if (request == null) {
            throw new IllegalArgumentException(
                "La solicitud no puede ser nula"
            );
        }

        if (request.getName() == null || request.getName().isBlank()) {
            throw new IllegalArgumentException(
                "El nombre del evento es obligatorio"
            );
        }

        if (
            request.getTotalCapacity() == null ||
            request.getTotalCapacity() <= 0
        ) {
            throw new IllegalArgumentException(
                "La capacidad total debe ser mayor a cero"
            );
        }

        if (request.getVenueId() == null) {
            throw new IllegalArgumentException("La sede es obligatoria");
        }

        if (request.getTicketPrice() == null) {
            throw new IllegalArgumentException(
                "El precio del ticket es obligatorio"
            );
        }

        if (request.getTicketPrice() <= 0) {
            throw new IllegalArgumentException(
                "El precio del ticket debe ser mayor a cero"
            );
        }
    }

    private void validateEventCapacityAgainstVenue(
        final CreateEventRequest request,
        final Venue venue
    ) {
        if (venue == null) {
            throw new IllegalArgumentException(
                "La sede seleccionada no existe"
            );
        }

        if (venue.getTotalCapacity() == null || venue.getTotalCapacity() <= 0) {
            throw new IllegalArgumentException(
                "La sede seleccionada no tiene una capacidad válida"
            );
        }

        if (request.getTotalCapacity() > venue.getTotalCapacity()) {
            throw new IllegalArgumentException(
                "La capacidad del evento no puede superar la capacidad total de la sede. " +
                    "Capacidad de la sede: " +
                    venue.getTotalCapacity()
            );
        }
    }

    public List<VenueInventoryResponse> getAllVenues() {
        try {
            List<Venue> venues = firebaseInventoryRepository
                .findAllVenues()
                .join();

            List<VenueInventoryResponse> response = new ArrayList<>();

            for (Venue venue : venues) {
                response.add(
                    VenueInventoryResponse.builder()
                        .venueId(venue.getId())
                        .venueName(venue.getName())
                        .address(venue.getAddress())
                        .totalCapacity(venue.getTotalCapacity())
                        .build()
                );
            }

            return response;
        } catch (Exception exception) {
            log.error("Error obteniendo sedes", exception);
            throw new RuntimeException("No fue posible consultar las sedes");
        }
    }

    public VenueInventoryResponse createVenue(
        final CreateVenueRequest request
    ) {
        validateCreateVenueRequest(request);

        try {
            Venue venue = Venue.builder()
                .id(System.currentTimeMillis())
                .name(request.getName())
                .address(request.getAddress())
                .totalCapacity(request.getTotalCapacity())
                .build();

            Venue savedVenue = firebaseInventoryRepository
                .saveVenue(venue)
                .join();

            return VenueInventoryResponse.builder()
                .venueId(savedVenue.getId())
                .venueName(savedVenue.getName())
                .address(savedVenue.getAddress())
                .totalCapacity(savedVenue.getTotalCapacity())
                .build();
        } catch (Exception exception) {
            log.error("Error creando sede", exception);
            throw new RuntimeException("No fue posible crear la sede");
        }
    }

    private void validateCreateVenueRequest(final CreateVenueRequest request) {
        if (request == null) {
            throw new IllegalArgumentException(
                "La solicitud no puede ser nula"
            );
        }

        if (request.getName() == null || request.getName().isBlank()) {
            throw new IllegalArgumentException(
                "El nombre de la sede es obligatorio"
            );
        }

        if (request.getAddress() == null || request.getAddress().isBlank()) {
            throw new IllegalArgumentException(
                "La dirección de la sede es obligatoria"
            );
        }

        if (
            request.getTotalCapacity() == null ||
            request.getTotalCapacity() <= 0
        ) {
            throw new IllegalArgumentException(
                "La capacidad total debe ser mayor a cero"
            );
        }
    }

    public void deleteEvent(final Long eventId) {
        if (eventId == null) {
            throw new IllegalArgumentException(
                "El id del evento es obligatorio"
            );
        }

        try {
            firebaseInventoryRepository.deleteEventById(eventId).join();

            log.info("Evento eliminado con id: {}", eventId);
        } catch (Exception exception) {
            log.error("Error eliminando evento con id: {}", eventId, exception);
            throw new RuntimeException("No fue posible eliminar el evento");
        }
    }

    public void deleteVenue(final Long venueId) {
        if (venueId == null) {
            throw new IllegalArgumentException(
                "El id de la sede es obligatorio"
            );
        }

        try {
            Venue venue = firebaseInventoryRepository
                .findVenueById(venueId)
                .join();

            List<Event> events = firebaseInventoryRepository
                .findAllEvents()
                .join();

            boolean venueHasEvents = events
                .stream()
                .anyMatch(event -> venueId.equals(event.getVenueId()));

            if (venueHasEvents) {
                throw new IllegalArgumentException(
                    "No se puede eliminar la sede porque tiene eventos asociados"
                );
            }

            firebaseInventoryRepository.deleteVenueById(venue.getId()).join();

            log.info("Sede eliminada con id: {}", venueId);
        } catch (IllegalArgumentException exception) {
            throw exception;
        } catch (Exception exception) {
            log.error("Error eliminando sede con id: {}", venueId, exception);
            throw new RuntimeException("No fue posible eliminar la sede");
        }
    }

    public VenueInventoryResponse updateVenue(
        final Long venueId,
        final UpdateVenueRequest request
    ) {
        if (venueId == null) {
            throw new IllegalArgumentException(
                "El id de la sede es obligatorio"
            );
        }

        validateUpdateVenueRequest(request);

        try {
            Venue existingVenue = firebaseInventoryRepository
                .findVenueById(venueId)
                .join();

            List<Event> events = firebaseInventoryRepository
                .findAllEvents()
                .join();

            boolean hasEventExceedingNewCapacity = events
                .stream()
                .filter(event -> venueId.equals(event.getVenueId()))
                .anyMatch(
                    event ->
                        event.getTotalCapacity() != null &&
                        event.getTotalCapacity() > request.getTotalCapacity()
                );

            if (hasEventExceedingNewCapacity) {
                throw new IllegalArgumentException(
                    "No se puede reducir la capacidad de la sede por debajo de la capacidad de eventos asociados"
                );
            }

            existingVenue.setName(request.getName());
            existingVenue.setAddress(request.getAddress());
            existingVenue.setTotalCapacity(request.getTotalCapacity());

            Venue updatedVenue = firebaseInventoryRepository
                .saveVenue(existingVenue)
                .join();

            return VenueInventoryResponse.builder()
                .venueId(updatedVenue.getId())
                .venueName(updatedVenue.getName())
                .address(updatedVenue.getAddress())
                .totalCapacity(updatedVenue.getTotalCapacity())
                .build();
        } catch (IllegalArgumentException exception) {
            throw exception;
        } catch (Exception exception) {
            log.error("Error actualizando sede con id: {}", venueId, exception);
            throw new RuntimeException("No fue posible actualizar la sede");
        }
    }

    private void validateUpdateVenueRequest(final UpdateVenueRequest request) {
        if (request == null) {
            throw new IllegalArgumentException(
                "La solicitud no puede ser nula"
            );
        }

        if (request.getName() == null || request.getName().isBlank()) {
            throw new IllegalArgumentException(
                "El nombre de la sede es obligatorio"
            );
        }

        if (request.getAddress() == null || request.getAddress().isBlank()) {
            throw new IllegalArgumentException(
                "La dirección de la sede es obligatoria"
            );
        }

        if (
            request.getTotalCapacity() == null ||
            request.getTotalCapacity() <= 0
        ) {
            throw new IllegalArgumentException(
                "La capacidad total debe ser mayor a cero"
            );
        }
    }
}
