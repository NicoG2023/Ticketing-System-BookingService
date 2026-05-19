package com.nicog.inventoryservice.repository;

import com.google.firebase.database.DataSnapshot;
import com.google.firebase.database.DatabaseError;
import com.google.firebase.database.DatabaseReference;
import com.google.firebase.database.FirebaseDatabase;
import com.google.firebase.database.MutableData;
import com.google.firebase.database.Transaction;
import com.google.firebase.database.ValueEventListener;
import com.nicog.inventoryservice.entity.Event;
import com.nicog.inventoryservice.entity.Venue;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import org.springframework.stereotype.Repository;

@Repository
public class FirebaseInventoryRepository {

    private final DatabaseReference eventsRef;
    private final DatabaseReference venuesRef;

    public FirebaseInventoryRepository() {
        FirebaseDatabase database = FirebaseDatabase.getInstance();
        this.eventsRef = database.getReference("events");
        this.venuesRef = database.getReference("venues");
    }

    public CompletableFuture<List<Event>> findAllEvents() {
        CompletableFuture<List<Event>> future = new CompletableFuture<>();

        eventsRef.addListenerForSingleValueEvent(
            new ValueEventListener() {
                @Override
                public void onDataChange(DataSnapshot snapshot) {
                    List<Event> events = new ArrayList<>();

                    for (DataSnapshot child : snapshot.getChildren()) {
                        Event event = child.getValue(Event.class);

                        if (event != null) {
                            events.add(event);
                        }
                    }

                    future.complete(events);
                }

                @Override
                public void onCancelled(DatabaseError error) {
                    future.completeExceptionally(
                        new RuntimeException(
                            "Error consultando eventos: " + error.getMessage()
                        )
                    );
                }
            }
        );

        return future;
    }

    public CompletableFuture<Event> findEventById(Long eventId) {
        CompletableFuture<Event> future = new CompletableFuture<>();

        eventsRef.child(String.valueOf(eventId)).addListenerForSingleValueEvent(
            new ValueEventListener() {
                @Override
                public void onDataChange(DataSnapshot snapshot) {
                    Event event = snapshot.getValue(Event.class);

                    if (event == null) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "No existe el evento con id: " + eventId
                            )
                        );
                        return;
                    }

                    future.complete(event);
                }

                @Override
                public void onCancelled(DatabaseError error) {
                    future.completeExceptionally(
                        new RuntimeException(
                            "Error consultando evento: " + error.getMessage()
                        )
                    );
                }
            }
        );

        return future;
    }

    public CompletableFuture<Venue> findVenueById(Long venueId) {
        CompletableFuture<Venue> future = new CompletableFuture<>();

        venuesRef.child(String.valueOf(venueId)).addListenerForSingleValueEvent(
            new ValueEventListener() {
                @Override
                public void onDataChange(DataSnapshot snapshot) {
                    Venue venue = snapshot.getValue(Venue.class);

                    if (venue == null) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "No existe la sede con id: " + venueId
                            )
                        );
                        return;
                    }

                    future.complete(venue);
                }

                @Override
                public void onCancelled(DatabaseError error) {
                    future.completeExceptionally(
                        new RuntimeException(
                            "Error consultando sede: " + error.getMessage()
                        )
                    );
                }
            }
        );

        return future;
    }

    public CompletableFuture<Void> decreaseEventCapacity(
        Long eventId,
        Long ticketsBooked
    ) {
        CompletableFuture<Void> future = new CompletableFuture<>();

        DatabaseReference eventRef = eventsRef.child(String.valueOf(eventId));

        eventRef.runTransaction(
            new Transaction.Handler() {
                @Override
                public Transaction.Result doTransaction(
                    MutableData currentData
                ) {
                    Event event = currentData.getValue(Event.class);

                    if (event == null) {
                        return Transaction.abort();
                    }

                    Long currentCapacity = event.getLeftCapacity();

                    if (
                        currentCapacity == null ||
                        currentCapacity < ticketsBooked
                    ) {
                        return Transaction.abort();
                    }

                    event.setLeftCapacity(currentCapacity - ticketsBooked);
                    currentData.setValue(event);

                    return Transaction.success(currentData);
                }

                @Override
                public void onComplete(
                    DatabaseError error,
                    boolean committed,
                    DataSnapshot currentData
                ) {
                    if (error != null) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "Error actualizando inventario: " +
                                    error.getMessage()
                            )
                        );
                        return;
                    }

                    if (!committed) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "No hay capacidad suficiente para el evento: " +
                                    eventId
                            )
                        );
                        return;
                    }

                    future.complete(null);
                }
            }
        );

        return future;
    }

    public CompletableFuture<Void> increaseEventCapacity(
        Long eventId,
        Long ticketsReleased
    ) {
        CompletableFuture<Void> future = new CompletableFuture<>();

        DatabaseReference eventRef = eventsRef.child(String.valueOf(eventId));

        eventRef.runTransaction(
            new Transaction.Handler() {
                @Override
                public Transaction.Result doTransaction(
                    MutableData currentData
                ) {
                    Event event = currentData.getValue(Event.class);

                    if (event == null) {
                        return Transaction.abort();
                    }

                    Long currentCapacity = event.getLeftCapacity();

                    if (currentCapacity == null) {
                        return Transaction.abort();
                    }

                    Long newCapacity = currentCapacity + ticketsReleased;

                    if (
                        event.getTotalCapacity() != null &&
                        newCapacity > event.getTotalCapacity()
                    ) {
                        return Transaction.abort();
                    }

                    event.setLeftCapacity(newCapacity);
                    currentData.setValue(event);

                    return Transaction.success(currentData);
                }

                @Override
                public void onComplete(
                    DatabaseError error,
                    boolean committed,
                    DataSnapshot currentData
                ) {
                    if (error != null) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "Error compensando inventario: " +
                                    error.getMessage()
                            )
                        );
                        return;
                    }

                    if (!committed) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "No fue posible compensar inventario para evento: " +
                                    eventId
                            )
                        );
                        return;
                    }

                    future.complete(null);
                }
            }
        );

        return future;
    }
}
