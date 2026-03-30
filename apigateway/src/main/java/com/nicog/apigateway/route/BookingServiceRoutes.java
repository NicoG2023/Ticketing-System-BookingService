package com.nicog.apigateway.route;

import static org.springframework.cloud.gateway.server.mvc.filter.BeforeFilterFunctions.uri;

import java.net.URI;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.cloud.gateway.server.mvc.filter.CircuitBreakerFilterFunctions;
import org.springframework.cloud.gateway.server.mvc.handler.GatewayRouterFunctions;
import org.springframework.cloud.gateway.server.mvc.handler.HandlerFunctions;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpStatus;
import org.springframework.web.servlet.function.RequestPredicates;
import org.springframework.web.servlet.function.RouterFunction;
import org.springframework.web.servlet.function.ServerResponse;

@Configuration
public class BookingServiceRoutes {

    @Value("${services.booking.url}")
    private String bookingServiceUrl;

    @Bean
    public RouterFunction<ServerResponse> bookingRoutes() {
        return GatewayRouterFunctions.route("booking-service")
            .route(
                RequestPredicates.POST("/api/v1/booking"),
                HandlerFunctions.http()
            )
            .filter(
                CircuitBreakerFilterFunctions.circuitBreaker(
                    "bookingServiceCircuitBreaker",
                    URI.create("forward:/fallbackRoute")
                )
            )
            .before(uri(URI.create(bookingServiceUrl)))
            .build();
    }

    @Bean
    public RouterFunction<ServerResponse> fallbackRoute() {
        return GatewayRouterFunctions.route("fallbackRoute")
            .POST("/fallbackRoute", request ->
                ServerResponse.status(HttpStatus.SERVICE_UNAVAILABLE).body(
                    "Booking service is down"
                )
            )
            .build();
    }
}
