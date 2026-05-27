package com.nicog.bookingservice.service;

import com.nicog.bookingservice.client.InventoryServiceClient;
import com.nicog.bookingservice.event.BookingEvent;
import com.nicog.bookingservice.request.BookingRequest;
import com.nicog.bookingservice.response.BookingResponse;
import com.nicog.bookingservice.response.InventoryResponse;
import lombok.extern.slf4j.Slf4j;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class BookingService {

    private final InventoryServiceClient inventoryServiceClient;
    private final KafkaTemplate<String, BookingEvent> kafkaTemplate;

    public BookingService(
        final InventoryServiceClient inventoryServiceClient,
        final KafkaTemplate<String, BookingEvent> kafkaTemplate
    ) {
        this.inventoryServiceClient = inventoryServiceClient;
        this.kafkaTemplate = kafkaTemplate;
    }

    public BookingResponse createBooking(
        final BookingRequest request,
        final String userId,
        final String email,
        final String username
    ) {
        validateBookingRequest(request);
        validateAuthenticatedUser(userId);

        log.info(
            "Creating booking for userId={}, email={}, username={}",
            userId,
            email,
            username
        );

        final InventoryResponse inventoryResponse =
            inventoryServiceClient.getInventory(request.getEventId());

        if (inventoryResponse == null) {
            throw new RuntimeException(
                "No fue posible consultar el inventario del evento"
            );
        }

        log.info("Inventory Response: {}", inventoryResponse);

        if (inventoryResponse.getCapacity() == null) {
            throw new RuntimeException(
                "El inventario del evento no tiene capacidad registrada"
            );
        }

        if (inventoryResponse.getCapacity() < request.getTicketCount()) {
            throw new RuntimeException("Not enough inventory");
        }

        final BookingEvent bookingEvent = createBookingEvent(
            request,
            userId,
            inventoryResponse
        );

        kafkaTemplate.send("booking", bookingEvent);

        log.info("Booking sent to Kafka: {}", bookingEvent);

        return BookingResponse.builder()
            .userId(bookingEvent.getUserId())
            .eventId(bookingEvent.getEventId())
            .ticketCount(bookingEvent.getTicketCount())
            .totalPrice(bookingEvent.getTotalPrice())
            .build();
    }

    private BookingEvent createBookingEvent(
        final BookingRequest request,
        final String userId,
        final InventoryResponse inventoryResponse
    ) {
        return BookingEvent.builder()
            .userId(userId)
            .eventId(request.getEventId())
            .ticketCount(request.getTicketCount())
            .totalPrice(
                inventoryResponse.getTicketPrice() * request.getTicketCount()
            )
            .build();
    }

    private void validateBookingRequest(final BookingRequest request) {
        if (request == null) {
            throw new IllegalArgumentException(
                "La solicitud de reserva no puede ser nula"
            );
        }

        if (request.getEventId() == null) {
            throw new IllegalArgumentException(
                "El id del evento es obligatorio"
            );
        }

        if (request.getTicketCount() == null || request.getTicketCount() <= 0) {
            throw new IllegalArgumentException(
                "La cantidad de tickets debe ser mayor a cero"
            );
        }
    }

    private void validateAuthenticatedUser(final String userId) {
        if (userId == null || userId.isBlank()) {
            throw new IllegalArgumentException(
                "No se encontró un usuario autenticado"
            );
        }
    }
}
