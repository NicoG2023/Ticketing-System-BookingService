package com.nicog.inventoryservice.service;

import com.nicog.inventoryservice.entity.Event;
import com.nicog.inventoryservice.entity.Venue;
import com.nicog.inventoryservice.repository.FirebaseInventoryRepository;
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
}
