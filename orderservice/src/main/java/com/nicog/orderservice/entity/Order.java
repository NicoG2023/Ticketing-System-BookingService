package com.nicog.orderservice.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import java.math.BigDecimal;
import java.time.LocalDateTime;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.hibernate.annotations.CreationTimestamp;

@Entity
@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
@Table(name = "orden")
public class Order {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(name = "precio_total")
    private BigDecimal totalPrice;

    @Column(name = "recuento_tickets")
    private Long ticketCount;

    @CreationTimestamp
    @Column(name = "placed_at", updatable = false, nullable = false)
    private LocalDateTime placedAt;

    @Column(name = "id_cliente")
    private Long customerId;

    @Column(name = "id_evento")
    private Long eventId;
}
