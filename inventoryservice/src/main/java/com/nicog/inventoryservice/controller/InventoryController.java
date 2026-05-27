package com.nicog.inventoryservice.controller;

import com.nicog.inventoryservice.request.CreateEventRequest;
import com.nicog.inventoryservice.request.CreateVenueRequest;
import com.nicog.inventoryservice.request.UpdateVenueRequest;
import com.nicog.inventoryservice.response.EventInventoryResponse;
import com.nicog.inventoryservice.response.LostUpdateSimulationResponse;
import com.nicog.inventoryservice.response.VenueInventoryResponse;
import com.nicog.inventoryservice.service.InventoryService;
import java.util.List;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
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

    @GetMapping("/events/{eventId}/simulations/lost-update")
    public ResponseEntity<LostUpdateSimulationResponse> simulateLostUpdate(
        @PathVariable Long eventId
    ) {
        return ResponseEntity.ok(inventoryService.simulateLostUpdate(eventId));
    }

    @PostMapping(
        consumes = "application/json",
        produces = "application/json",
        path = "/inventory/events"
    )
    public ResponseEntity<EventInventoryResponse> createEvent(
        @RequestBody CreateEventRequest request
    ) {
        return ResponseEntity.ok(inventoryService.createEvent(request));
    }

    @GetMapping("/inventory/venues")
    public ResponseEntity<
        List<VenueInventoryResponse>
    > inventoryGetAllVenues() {
        return ResponseEntity.ok(inventoryService.getAllVenues());
    }

    @PostMapping(
        consumes = "application/json",
        produces = "application/json",
        path = "/inventory/venues"
    )
    public ResponseEntity<VenueInventoryResponse> createVenue(
        @RequestBody CreateVenueRequest request
    ) {
        return ResponseEntity.ok(inventoryService.createVenue(request));
    }

    @DeleteMapping("/inventory/event/{eventId}")
    public ResponseEntity<Void> deleteEvent(@PathVariable Long eventId) {
        inventoryService.deleteEvent(eventId);
        return ResponseEntity.noContent().build();
    }

    @DeleteMapping("/inventory/venue/{venueId}")
    public ResponseEntity<Void> deleteVenue(@PathVariable Long venueId) {
        inventoryService.deleteVenue(venueId);
        return ResponseEntity.noContent().build();
    }

    @PutMapping(
        consumes = "application/json",
        produces = "application/json",
        path = "/inventory/venue/{venueId}"
    )
    public ResponseEntity<VenueInventoryResponse> updateVenue(
        @PathVariable Long venueId,
        @RequestBody UpdateVenueRequest request
    ) {
        return ResponseEntity.ok(
            inventoryService.updateVenue(venueId, request)
        );
    }
}
