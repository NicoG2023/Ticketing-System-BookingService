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
import com.nicog.inventoryservice.simulation.LostUpdateSimulationState;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class InventoryService {

    private static final Long TICKETS_TO_BOOK = 1L;
    private final Map<Long, LostUpdateSimulationState> lostUpdateStates =
        new ConcurrentHashMap<>();

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

    public LostUpdateSimulationResponse startLostUpdateSimulation(
        final Long eventId
    ) {
        try {
            Event event = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            Long initialCapacity = event.getLeftCapacity();

            if (initialCapacity == null) {
                throw new RuntimeException(
                    "El evento no tiene capacidad disponible registrada"
                );
            }

            LostUpdateSimulationState state =
                LostUpdateSimulationState.builder()
                    .eventId(eventId)
                    .initialCapacity(initialCapacity)
                    .requestACommitted(false)
                    .requestBCommitted(false)
                    .requestAStatus("Pendiente")
                    .requestBStatus("Pendiente")
                    .build();

            lostUpdateStates.put(eventId, state);

            return buildLostUpdateResponse(state);
        } catch (Exception exception) {
            throw new RuntimeException(
                "No fue posible iniciar la simulación de Lost Update",
                exception
            );
        }
    }

    public LostUpdateSimulationResponse readLostUpdateCapacity(
        final Long eventId,
        final String session
    ) {
        LostUpdateSimulationState state = getLostUpdateState(eventId);

        try {
            Event event = firebaseInventoryRepository
                .findEventById(eventId)
                .join();

            Long capacityRead = event.getLeftCapacity();

            if (capacityRead == null) {
                throw new RuntimeException(
                    "El evento no tiene capacidad disponible registrada"
                );
            }

            if (isSessionA(session)) {
                state.setRequestAReadCapacity(capacityRead);
                state.setRequestAStatus("A leyó capacidad: " + capacityRead);
            } else if (isSessionB(session)) {
                state.setRequestBReadCapacity(capacityRead);
                state.setRequestBStatus("B leyó capacidad: " + capacityRead);
            } else {
                throw new IllegalArgumentException(
                    "Sesión inválida: " + session
                );
            }

            return buildLostUpdateResponse(state);
        } catch (Exception exception) {
            throw new RuntimeException(
                "No fue posible leer la capacidad en la simulación",
                exception
            );
        }
    }

    public LostUpdateSimulationResponse calculateLostUpdateCapacity(
        final Long eventId,
        final String session
    ) {
        LostUpdateSimulationState state = getLostUpdateState(eventId);

        if (isSessionA(session)) {
            if (state.getRequestAReadCapacity() == null) {
                throw new IllegalStateException(
                    "La sesión A debe leer la capacidad antes de calcular"
                );
            }

            Long calculatedCapacity =
                state.getRequestAReadCapacity() - TICKETS_TO_BOOK;

            state.setRequestACalculatedCapacity(calculatedCapacity);
            state.setRequestAStatus(
                "A calculó nueva capacidad: " + calculatedCapacity
            );
        } else if (isSessionB(session)) {
            if (state.getRequestBReadCapacity() == null) {
                throw new IllegalStateException(
                    "La sesión B debe leer la capacidad antes de calcular"
                );
            }

            Long calculatedCapacity =
                state.getRequestBReadCapacity() - TICKETS_TO_BOOK;

            state.setRequestBCalculatedCapacity(calculatedCapacity);
            state.setRequestBStatus(
                "B calculó nueva capacidad: " + calculatedCapacity
            );
        } else {
            throw new IllegalArgumentException("Sesión inválida: " + session);
        }

        return buildLostUpdateResponse(state);
    }

    public LostUpdateSimulationResponse commitLostUpdateCapacity(
        final Long eventId,
        final String session
    ) {
        LostUpdateSimulationState state = getLostUpdateState(eventId);

        try {
            if (isSessionA(session)) {
                if (state.getRequestACalculatedCapacity() == null) {
                    throw new IllegalStateException(
                        "La sesión A debe calcular antes de guardar"
                    );
                }

                firebaseInventoryRepository
                    .unsafeSetLeftCapacity(
                        eventId,
                        state.getRequestACalculatedCapacity()
                    )
                    .join();

                state.setRequestACommitted(true);
                state.setRequestAStatus(
                    "A guardó capacidad: " +
                        state.getRequestACalculatedCapacity()
                );
            } else if (isSessionB(session)) {
                if (state.getRequestBCalculatedCapacity() == null) {
                    throw new IllegalStateException(
                        "La sesión B debe calcular antes de guardar"
                    );
                }

                firebaseInventoryRepository
                    .unsafeSetLeftCapacity(
                        eventId,
                        state.getRequestBCalculatedCapacity()
                    )
                    .join();

                state.setRequestBCommitted(true);
                state.setRequestBStatus(
                    "B guardó capacidad: " +
                        state.getRequestBCalculatedCapacity()
                );
            } else {
                throw new IllegalArgumentException(
                    "Sesión inválida: " + session
                );
            }

            return buildLostUpdateResponse(state);
        } catch (Exception exception) {
            throw new RuntimeException(
                "No fue posible guardar la capacidad en la simulación",
                exception
            );
        }
    }

    public LostUpdateSimulationResponse restoreLostUpdateSimulation(
        final Long eventId
    ) {
        LostUpdateSimulationState state = getLostUpdateState(eventId);

        try {
            firebaseInventoryRepository
                .unsafeSetLeftCapacity(eventId, state.getInitialCapacity())
                .join();

            LostUpdateSimulationState restoredState =
                LostUpdateSimulationState.builder()
                    .eventId(eventId)
                    .initialCapacity(state.getInitialCapacity())
                    .requestACommitted(false)
                    .requestBCommitted(false)
                    .requestAStatus("Pendiente")
                    .requestBStatus("Pendiente")
                    .build();

            lostUpdateStates.put(eventId, restoredState);

            return buildLostUpdateResponse(restoredState);
        } catch (Exception exception) {
            throw new RuntimeException(
                "No fue posible restaurar la simulación",
                exception
            );
        }
    }

    private LostUpdateSimulationState getLostUpdateState(final Long eventId) {
        LostUpdateSimulationState state = lostUpdateStates.get(eventId);

        if (state == null) {
            throw new IllegalStateException(
                "Primero debes iniciar la simulación para el evento: " + eventId
            );
        }

        return state;
    }

    private boolean isSessionA(final String session) {
        return "A".equalsIgnoreCase(session);
    }

    private boolean isSessionB(final String session) {
        return "B".equalsIgnoreCase(session);
    }

    private LostUpdateSimulationResponse buildLostUpdateResponse(
        final LostUpdateSimulationState state
    ) {
        Long finalCapacity = null;

        try {
            Event event = firebaseInventoryRepository
                .findEventById(state.getEventId())
                .join();

            finalCapacity = event.getLeftCapacity();
        } catch (Exception exception) {
            log.warn(
                "No fue posible consultar capacidad final para simulación Lost Update. eventId={}",
                state.getEventId(),
                exception
            );
        }

        Long expectedCapacity = state.getInitialCapacity() - 2;

        boolean bothRequestsCommitted =
            Boolean.TRUE.equals(state.getRequestACommitted()) &&
            Boolean.TRUE.equals(state.getRequestBCommitted());

        boolean lostUpdateOccurred =
            bothRequestsCommitted &&
            finalCapacity != null &&
            !expectedCapacity.equals(finalCapacity);

        return LostUpdateSimulationResponse.builder()
            .eventId(state.getEventId())
            .initialCapacity(state.getInitialCapacity())
            .requestAReadCapacity(state.getRequestAReadCapacity())
            .requestACalculatedCapacity(state.getRequestACalculatedCapacity())
            .requestBReadCapacity(state.getRequestBReadCapacity())
            .requestBCalculatedCapacity(state.getRequestBCalculatedCapacity())
            .requestACommitted(state.getRequestACommitted())
            .requestBCommitted(state.getRequestBCommitted())
            .finalCapacity(finalCapacity)
            .expectedCapacity(expectedCapacity)
            .lostUpdateOccurred(lostUpdateOccurred)
            .requestAStatus(state.getRequestAStatus())
            .requestBStatus(state.getRequestBStatus())
            .explanation(
                "El Lost Update ocurre cuando dos reservas leen la misma capacidad " +
                    "antes de que alguna guarde el cambio. Cada una calcula una nueva " +
                    "capacidad con base en el valor viejo, y al guardar, una escritura " +
                    "sobrescribe a la otra."
            )
            .control(
                "Este problema se controla usando una transacción. En este proyecto, " +
                    "el método decreaseEventCapacity usa runTransaction para que lectura " +
                    "y escritura ocurran como una operación atómica."
            )
            .build();
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
