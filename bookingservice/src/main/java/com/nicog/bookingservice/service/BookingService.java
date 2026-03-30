package com.nicog.bookingservice.service;

import com.nicog.bookingservice.client.InventoryServiceClient;
import com.nicog.bookingservice.entity.Customer;
import com.nicog.bookingservice.event.BookingEvent;
import com.nicog.bookingservice.repository.CustomerRepository;
import com.nicog.bookingservice.request.BookingRequest;
import com.nicog.bookingservice.response.BookingResponse;
import com.nicog.bookingservice.response.InventoryResponse;
import java.math.BigDecimal;
import lombok.extern.slf4j.Slf4j;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class BookingService {

    private final CustomerRepository customerRepository;
    private final InventoryServiceClient inventoryServiceClient;
    private final KafkaTemplate<String, BookingEvent> kafkaTemplate;

    public BookingService(
        final CustomerRepository customerRepository,
        final InventoryServiceClient inventoryServiceClient,
        final KafkaTemplate<String, BookingEvent> kafkaTemplate
    ) {
        this.customerRepository = customerRepository;
        this.inventoryServiceClient = inventoryServiceClient;
        this.kafkaTemplate = kafkaTemplate;
    }

    public BookingResponse createBooking(final BookingRequest request) {
        // check if user exists
        final Customer customer = customerRepository
            .findById(request.getUserId())
            .orElse(null);
        if (customer == null) {
            throw new RuntimeException("User not found");
        }
        // check if there is enough inventory
        final InventoryResponse inventoryResponse =
            inventoryServiceClient.getInventory(request.getEventId());
        log.info("Inventory Response: {}", inventoryResponse);
        if (inventoryResponse.getCapacity() < request.getTicketCount()) {
            throw new RuntimeException("Not enough inventory");
        }
        // -- get event information to also get Venue information
        // create booking
        final BookingEvent bookingEvent = createBookingEvent(
            request,
            customer,
            inventoryResponse
        );
        // send booking to Order Service on a Kafka Topic
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
        final Customer customer,
        final InventoryResponse inventoryResponse
    ) {
        return BookingEvent.builder()
            .userId(request.getUserId())
            .eventId(request.getEventId())
            .ticketCount(request.getTicketCount())
            .totalPrice(
                inventoryResponse
                    .getTicketPrice()
                    .multiply(BigDecimal.valueOf(request.getTicketCount()))
            )
            .build();
    }
}
