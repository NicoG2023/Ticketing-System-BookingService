package com.nicog.orderservice.service;

import com.example.nicog.bookingservice.event.BookingEvent;
import com.nicog.orderservice.client.InventoryServiceClient;
import com.nicog.orderservice.entity.Order;
import com.nicog.orderservice.repository.FirebaseOrderRepository;
import java.time.LocalDateTime;
import lombok.extern.slf4j.Slf4j;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class OrderService {

    private final FirebaseOrderRepository orderRepository;
    private final InventoryServiceClient inventoryServiceClient;

    public OrderService(
        final FirebaseOrderRepository orderRepository,
        final InventoryServiceClient inventoryServiceClient
    ) {
        this.orderRepository = orderRepository;
        this.inventoryServiceClient = inventoryServiceClient;
    }

    @KafkaListener(topics = "booking", groupId = "order-service")
    public void orderEvent(BookingEvent bookingEvent) {
        log.info("Received order event: {}", bookingEvent);

        boolean inventoryUpdated = false;

        try {
            validateBookingEvent(bookingEvent);

            inventoryServiceClient.updateInventory(
                bookingEvent.getEventId(),
                bookingEvent.getTicketCount()
            );

            inventoryUpdated = true;

            log.info(
                "Inventory updated for event: {}, less tickets: {}",
                bookingEvent.getEventId(),
                bookingEvent.getTicketCount()
            );

            Order order = createOrder(bookingEvent);

            Order savedOrder = orderRepository.save(order).join();

            log.info("Order saved successfully: {}", savedOrder);
        } catch (Exception exception) {
            log.error(
                "Order creation failed for booking event: {}. Reason: {}",
                bookingEvent,
                exception.getMessage(),
                exception
            );

            if (inventoryUpdated) {
                compensateInventory(bookingEvent);
            }
        }
    }

    private Order createOrder(BookingEvent bookingEvent) {
        return Order.builder()
            .userId(bookingEvent.getUserId())
            .eventId(bookingEvent.getEventId())
            .ticketCount(bookingEvent.getTicketCount())
            .totalPrice(bookingEvent.getTotalPrice())
            .placedAt(LocalDateTime.now().toString())
            .status("CONFIRMED")
            .build();
    }

    private void validateBookingEvent(BookingEvent bookingEvent) {
        if (bookingEvent == null) {
            throw new IllegalArgumentException(
                "El evento de reserva no puede ser nulo"
            );
        }

        if (
            bookingEvent.getUserId() == null ||
            bookingEvent.getUserId().isBlank()
        ) {
            throw new IllegalArgumentException(
                "El id del usuario es obligatorio"
            );
        }

        if (bookingEvent.getEventId() == null) {
            throw new IllegalArgumentException(
                "El id del evento es obligatorio"
            );
        }

        if (
            bookingEvent.getTicketCount() == null ||
            bookingEvent.getTicketCount() <= 0
        ) {
            throw new IllegalArgumentException(
                "La cantidad de tickets debe ser mayor a cero"
            );
        }

        if (bookingEvent.getTotalPrice() == null) {
            throw new IllegalArgumentException(
                "El precio total es obligatorio"
            );
        }
    }

    private void compensateInventory(BookingEvent bookingEvent) {
        try {
            inventoryServiceClient.releaseInventory(
                bookingEvent.getEventId(),
                bookingEvent.getTicketCount()
            );

            log.info(
                "Inventory compensation executed for event: {}, returned tickets: {}",
                bookingEvent.getEventId(),
                bookingEvent.getTicketCount()
            );
        } catch (Exception compensationException) {
            log.error(
                "Critical error: inventory compensation failed for event: {}, tickets: {}",
                bookingEvent.getEventId(),
                bookingEvent.getTicketCount(),
                compensationException
            );
        }
    }
}
