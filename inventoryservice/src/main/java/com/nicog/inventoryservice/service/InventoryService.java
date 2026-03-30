package com.nicog.inventoryservice.service;

import com.nicog.inventoryservice.entity.Event;
import com.nicog.inventoryservice.entity.Venue;
import com.nicog.inventoryservice.repository.EventRepository;
import com.nicog.inventoryservice.repository.VenueRepository;
import com.nicog.inventoryservice.response.EventInventoryResponse;
import com.nicog.inventoryservice.response.VenueInventoryResponse;
import java.util.List;
import java.util.stream.Collectors;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

@Service
@Slf4j
public class InventoryService {

    private final EventRepository eventRepository;
    private final VenueRepository venueRepository;

    public InventoryService(
        final EventRepository eventRepository,
        final VenueRepository venueRepository
    ) {
        this.eventRepository = eventRepository;
        this.venueRepository = venueRepository;
    }

    public List<EventInventoryResponse> getAllEvents() {
        final List<Event> events = eventRepository.findAll(); //Se devuelve una lista de objetos Event

        //el .stream convierte la lista en un flujo de elementos para poder procesarlos uno a uno
        //el .map toma cada elemento y lo transforma. Cada Event se convierte en un EventInventoryResponse
        //el .collect junta todos esos resultados y los mete en una List
        return events
            .stream()
            .map(event ->
                EventInventoryResponse.builder()
                    .event(event.getName())
                    .capacity(event.getLeftCapacity())
                    .venue(event.getVenue())
                    .build()
            )
            .collect(Collectors.toList());
    }

    public VenueInventoryResponse getVenueInformation(final Long venueId) {
        final Venue venue = venueRepository.findById(venueId).orElse(null); //el findById devuelve un Optional<Venue>

        //como no se trabaja con lista, sino con un objeto, se hace conversion directa de Venue a VenueInventoryResponse
        return VenueInventoryResponse.builder()
            .venueId(venue.getId())
            .venueName(venue.getName())
            .totalCapacity(venue.getTotalCapacity())
            .build();
    }

    public EventInventoryResponse getEventInventory(final Long eventId) {
        final Event event = eventRepository.findById(eventId).orElse(null);
        return EventInventoryResponse.builder()
            .event(event.getName())
            .capacity(event.getLeftCapacity())
            .venue(event.getVenue())
            .ticketPrice(event.getTicketPrice())
            .eventId(event.getId())
            .build();
    }

    public void updateEventCapacity(
        final Long eventId,
        final Long ticketsBooked
    ) {
        final Event event = eventRepository.findById(eventId).orElse(null);
        event.setLeftCapacity(event.getLeftCapacity() - ticketsBooked);
        eventRepository.saveAndFlush(event);
        log.info(
            "Updated event capacity for event id: {} with tickets booked: {}",
            eventId,
            ticketsBooked
        );
    }
}
