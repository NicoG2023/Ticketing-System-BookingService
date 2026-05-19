package com.nicog.orderservice.client;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;

@Service
public class InventoryServiceClient {

    private final RestTemplate restTemplate;

    @Value("${inventory.service.url}")
    private String inventoryServiceUrl;

    public InventoryServiceClient() {
        this.restTemplate = new RestTemplate();
    }

    public void updateInventory(final Long eventId, final Long ticketCount) {
        String url =
            inventoryServiceUrl +
            "/event/" +
            eventId +
            "/capacity/" +
            ticketCount;

        restTemplate.put(url, null);
    }

    public void releaseInventory(final Long eventId, final Long ticketCount) {
        String url =
            inventoryServiceUrl +
            "/event/" +
            eventId +
            "/capacity/release/" +
            ticketCount;

        restTemplate.put(url, null);
    }
}
