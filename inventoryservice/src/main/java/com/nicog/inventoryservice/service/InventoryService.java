package com.nicog.inventoryservice.service;

import com.nicog.inventoryservice.entity.Event;
import com.nicog.inventoryservice.entity.Venue;
import com.nicog.inventoryservice.repository.FirebaseInventoryRepository;
import com.nicog.inventoryservice.request.CreateEventRequest;
import com.nicog.inventoryservice.request.CreateVenueRequest;
import com.nicog.inventoryservice.response.ConcurrentBookingSimulationResponse;
import com.nicog.inventoryservice.response.EventInventoryResponse;
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
            firebaseInventoryRepository
                .decreaseEventCapacity(eventId, ticketsBooked)
                .join();

            log.info(
                "Updated event capacity for event id: {} with tickets booked: {}",
                eventId,
                ticketsBooked
            );
        } catch (Exception exception) {
            log.error(
                "Error actualizando capacidad del evento: {} con tickets: {}",
                eventId,
                ticketsBooked,
                exception
            );

            throw new RuntimeException(
                "No fue posible actualizar la capacidad del evento"
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

    public ConcurrentBookingSimulationResponse simulateConcurrentBooking(
        final Long eventId
    ) {
        final Long ticketsToBook = 1L;

        try {
            Event eventBefore = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            Long initialCapacity = eventBefore.getLeftCapacity();

            CompletableFuture<String> requestA = CompletableFuture.supplyAsync(
                () -> {
                    try {
                        firebaseInventoryRepository
                            .decreaseEventCapacity(eventId, ticketsToBook)
                            .join();

                        return "SUCCESS";
                    } catch (Exception exception) {
                        return "FAILED: " + exception.getMessage();
                    }
                }
            );

            CompletableFuture<String> requestB = CompletableFuture.supplyAsync(
                () -> {
                    try {
                        firebaseInventoryRepository
                            .decreaseEventCapacity(eventId, ticketsToBook)
                            .join();

                        return "SUCCESS";
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

            String requestAStatus = requestA.join();
            String requestBStatus = requestB.join();

            return ConcurrentBookingSimulationResponse.builder()
                .eventId(eventId)
                .initialCapacity(initialCapacity)
                .finalCapacity(finalCapacity)
                .requestAStatus(requestAStatus)
                .requestBStatus(requestBStatus)
                .conclusion(
                    "La simulación ejecutó dos reservas concurrentes. " +
                        "Gracias a la transacción de Firebase, no se permite que la capacidad quede negativa."
                )
                .build();
        } catch (Exception exception) {
            throw new RuntimeException(
                "No fue posible ejecutar la simulación de concurrencia",
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
}
