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

            // Liberar inventario / compensación
            .route(
                RequestPredicates.path(
                    "/api/v1/inventory/event/{eventId}/capacity/release/{ticketsReleased}"
                ),
                HandlerFunctions.http()
            )

            // Descontar inventario
            .route(
                RequestPredicates.path(
                    "/api/v1/inventory/event/{eventId}/capacity/{capacity}"
                ),
                HandlerFunctions.http()
            )

            // Consultar todas las sedes
            .route(
                RequestPredicates.path("/api/v1/inventory/venues"),
                HandlerFunctions.http()
            )

            // Consultar sede
            .route(
                RequestPredicates.path("/api/v1/inventory/venue/{venueId}"),
                HandlerFunctions.http()
            )

            // Consultar evento específico
            .route(
                RequestPredicates.path("/api/v1/inventory/event/{eventId}"),
                HandlerFunctions.http()
            )

            // Consultar todos los eventos
            .route(
                RequestPredicates.path("/api/v1/inventory/events"),
                HandlerFunctions.http()
            )

            // Simular reserva concurrente para simulacion de Lost Update
            .route(
                RequestPredicates.path(
                    "/api/v1/inventory/event/{eventId}/simulate-concurrent-booking"
                ),
                HandlerFunctions.http()
            )

            // Ruta para crear un evento
            .route(
                RequestPredicates.path("/api/v1/inventory/events"),
                HandlerFunctions.http()
            )

            // Ruta para crear una sede
            .route(
                RequestPredicates.path("/api/v1/inventory/venues"),
                HandlerFunctions.http()
            )

            .before(uri(URI.create(inventoryServiceUrl)))
            .build();
    }
}
