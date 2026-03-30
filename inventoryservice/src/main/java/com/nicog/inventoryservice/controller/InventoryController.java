package com.nicog.inventoryservice.controller;

import com.nicog.inventoryservice.response.EventInventoryResponse;
import com.nicog.inventoryservice.response.VenueInventoryResponse;
import com.nicog.inventoryservice.service.InventoryService;
import java.util.List;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
//RestController incluye @Controller y @ResponseBody, por que a lo largo de los endpoints, no es necesario agregar estas anotaciones
@RequestMapping("/api/v1")
public class InventoryController {

    private final InventoryService inventoryService;

    //Antes, si se necesitaba agregar la anotacion @Autowired para inyeccion de dependencias. Pero en este momento
    //Seria innecesario ya que tengo solo UN constructor.
    //El final en el parametro es una buena practica porque indica que no se puede reasignar el parametro o variable
    //esto evita errores accidentales, hace el codigo mas seguro, y mejora legibilidad
    public InventoryController(final InventoryService inventoryService) {
        this.inventoryService = inventoryService;
    }

    @GetMapping("/inventory/events")
    public List<EventInventoryResponse> inventoryGetAllEvents() {
        return inventoryService.getAllEvents();
    }

    @GetMapping("/inventory/venue/{venueId}")
    public VenueInventoryResponse inventoryByVenueId(
        @PathVariable Long venueId
    ) {
        return inventoryService.getVenueInformation(venueId);
    }

    @GetMapping("/inventory/event/{eventId}")
    public @ResponseBody EventInventoryResponse inventoryForEvent(
        @PathVariable("eventId") Long eventId
    ) {
        return inventoryService.getEventInventory(eventId);
    }

    @PutMapping("inventory/event/{eventId}/capacity/{capacity}")
    public ResponseEntity<Void> updateEventCapacity(
        @PathVariable Long eventId,
        @PathVariable Long capacity
    ) {
        inventoryService.updateEventCapacity(eventId, capacity);
        return ResponseEntity.ok().build();
    }
}
