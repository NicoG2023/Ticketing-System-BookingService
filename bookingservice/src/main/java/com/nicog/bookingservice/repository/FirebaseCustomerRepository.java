package com.nicog.bookingservice.repository;

import com.google.firebase.database.DataSnapshot;
import com.google.firebase.database.DatabaseError;
import com.google.firebase.database.DatabaseReference;
import com.google.firebase.database.FirebaseDatabase;
import com.google.firebase.database.ValueEventListener;
import com.nicog.bookingservice.entity.Customer;
import java.util.concurrent.CompletableFuture;
import org.springframework.stereotype.Repository;

@Repository
public class FirebaseCustomerRepository {

    private final DatabaseReference customersRef;

    public FirebaseCustomerRepository() {
        FirebaseDatabase database = FirebaseDatabase.getInstance();
        this.customersRef = database.getReference("customers");
    }

    public CompletableFuture<Customer> findById(Long customerId) {
        CompletableFuture<Customer> future = new CompletableFuture<>();

        customersRef
            .child(String.valueOf(customerId))
            .addListenerForSingleValueEvent(
                new ValueEventListener() {
                    @Override
                    public void onDataChange(DataSnapshot snapshot) {
                        Customer customer = snapshot.getValue(Customer.class);

                        if (customer == null) {
                            future.completeExceptionally(
                                new RuntimeException(
                                    "No existe el cliente con id: " + customerId
                                )
                            );
                            return;
                        }

                        future.complete(customer);
                    }

                    @Override
                    public void onCancelled(DatabaseError error) {
                        future.completeExceptionally(
                            new RuntimeException(
                                "Error consultando cliente: " +
                                    error.getMessage()
                            )
                        );
                    }
                }
            );

        return future;
    }
}
