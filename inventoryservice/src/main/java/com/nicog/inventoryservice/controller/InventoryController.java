package com.nicog.inventoryservice.controller;

import com.nicog.inventoryservice.response.ConcurrentBookingSimulationResponse;
import com.nicog.inventoryservice.response.EventInventoryResponse;
import com.nicog.inventoryservice.response.VenueInventoryResponse;
import com.nicog.inventoryservice.service.InventoryService;
import java.util.List;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1")
public class InventoryController {

    private final InventoryService inventoryService;

    public InventoryController(final InventoryService inventoryService) {
        this.inventoryService = inventoryService;
    }

    @GetMapping("/inventory/events")
    public ResponseEntity<
        List<EventInventoryResponse>
    > inventoryGetAllEvents() {
        return ResponseEntity.ok(inventoryService.getAllEvents());
    }

    @GetMapping("/inventory/venue/{venueId}")
    public ResponseEntity<VenueInventoryResponse> inventoryByVenueId(
        @PathVariable Long venueId
    ) {
        return ResponseEntity.ok(inventoryService.getVenueInformation(venueId));
    }

    @GetMapping("/inventory/event/{eventId}")
    public ResponseEntity<EventInventoryResponse> inventoryForEvent(
        @PathVariable Long eventId
    ) {
        return ResponseEntity.ok(inventoryService.getEventInventory(eventId));
    }

    @PutMapping("/inventory/event/{eventId}/capacity/{capacity}")
    public ResponseEntity<Void> updateEventCapacity(
        @PathVariable Long eventId,
        @PathVariable Long capacity
    ) {
        inventoryService.updateEventCapacity(eventId, capacity);
        return ResponseEntity.ok().build();
    }

    @PutMapping("/inventory/event/{eventId}/capacity/release/{ticketsReleased}")
    public ResponseEntity<Void> releaseEventCapacity(
        @PathVariable Long eventId,
        @PathVariable Long ticketsReleased
    ) {
        inventoryService.releaseEventCapacity(eventId, ticketsReleased);
        return ResponseEntity.ok().build();
    }

    @GetMapping("/inventory/event/{eventId}/simulate-concurrent-booking")
    public ResponseEntity<
        ConcurrentBookingSimulationResponse
    > simulateConcurrentBooking(@PathVariable Long eventId) {
        return ResponseEntity.ok(
            inventoryService.simulateConcurrentBooking(eventId)
        );
    }
}
