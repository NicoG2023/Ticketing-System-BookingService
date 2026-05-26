package com.nicog.apigateway.route;

import static org.springframework.cloud.gateway.server.mvc.filter.BeforeFilterFunctions.uri;

import java.net.URI;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.cloud.gateway.server.mvc.handler.GatewayRouterFunctions;
import org.springframework.cloud.gateway.server.mvc.handler.HandlerFunctions;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.function.RequestPredicates;
import org.springframework.web.servlet.function.RouterFunction;
import org.springframework.web.servlet.function.ServerResponse;

@Configuration
public class InventoryServiceRoutes {

    @Value("${services.inventory.url}")
    private String inventoryServiceUrl;

    @Bean
    public RouterFunction<ServerResponse> inventoryRoutes() {
        return GatewayRouterFunctions.route("inventory-service")

            // Eventos - lectura
            .route(
                RequestPredicates.GET("/api/v1/inventory/events"),
                HandlerFunctions.http()
            )
            .route(
                RequestPredicates.GET("/api/v1/inventory/event/{eventId}"),
                HandlerFunctions.http()
            )

            // Eventos - administración
            .route(
                RequestPredicates.POST("/api/v1/inventory/events"),
                HandlerFunctions.http()
            )
            .route(
                RequestPredicates.DELETE("/api/v1/inventory/event/{eventId}"),
                HandlerFunctions.http()
            )

            // Inventario de eventos
            .route(
                RequestPredicates.PUT(
                    "/api/v1/inventory/event/{eventId}/capacity/{capacity}"
                ),
                HandlerFunctions.http()
            )
            .route(
                RequestPredicates.PUT(
                    "/api/v1/inventory/event/{eventId}/capacity/release/{ticketsReleased}"
                ),
                HandlerFunctions.http()
            )

            // Simulación
            .route(
                RequestPredicates.GET(
                    "/api/v1/inventory/event/{eventId}/simulate-concurrent-booking"
                ),
                HandlerFunctions.http()
            )

            // Sedes - lectura
            .route(
                RequestPredicates.GET("/api/v1/inventory/venues"),
                HandlerFunctions.http()
            )
            .route(
                RequestPredicates.GET("/api/v1/inventory/venue/{venueId}"),
                HandlerFunctions.http()
            )

            // Sedes - administración
            .route(
                RequestPredicates.POST("/api/v1/inventory/venues"),
                HandlerFunctions.http()
            )
            .route(
                RequestPredicates.PUT("/api/v1/inventory/venue/{venueId}"),
                HandlerFunctions.http()
            )
            .route(
                RequestPredicates.DELETE("/api/v1/inventory/venue/{venueId}"),
                HandlerFunctions.http()
            )

            .before(uri(URI.create(inventoryServiceUrl)))
            .build();
    }
}
