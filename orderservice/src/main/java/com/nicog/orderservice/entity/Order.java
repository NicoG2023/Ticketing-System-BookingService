package com.nicog.orderservice.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class Order {

    private String id;
    private Double totalPrice;
    private Long ticketCount;
    private String placedAt;
    private String userId;
    private Long eventId;
    private String status;
}
