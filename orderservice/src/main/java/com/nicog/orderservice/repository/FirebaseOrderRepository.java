package com.nicog.orderservice.repository;

import com.google.firebase.database.DatabaseReference;
import com.google.firebase.database.FirebaseDatabase;
import com.nicog.orderservice.entity.Order;
import java.util.concurrent.CompletableFuture;
import org.springframework.stereotype.Repository;

@Repository
public class FirebaseOrderRepository {

    private final DatabaseReference ordersRef;

    public FirebaseOrderRepository() {
        FirebaseDatabase database = FirebaseDatabase.getInstance();
        this.ordersRef = database.getReference("orders");
    }

    public CompletableFuture<Order> save(Order order) {
        CompletableFuture<Order> future = new CompletableFuture<>();

        String generatedId = ordersRef.push().getKey();

        if (generatedId == null) {
            future.completeExceptionally(
                new RuntimeException("No fue posible generar el id de la orden")
            );
            return future;
        }

        order.setId(generatedId);

        ordersRef
            .child(generatedId)
            .setValue(order, (databaseError, databaseReference) -> {
                if (databaseError != null) {
                    future.completeExceptionally(
                        new RuntimeException(
                            "Error guardando orden: " +
                                databaseError.getMessage()
                        )
                    );
                    return;
                }

                future.complete(order);
            });

        return future;
    }
}
