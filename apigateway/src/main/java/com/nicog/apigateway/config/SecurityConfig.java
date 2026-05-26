package com.nicog.apigateway.config;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.security.config.Customizer;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.oauth2.jwt.JwtDecoder;
import org.springframework.security.oauth2.jwt.NimbusJwtDecoder;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationConverter;
import org.springframework.security.web.SecurityFilterChain;

@Configuration
public class SecurityConfig {

    @Value("${spring.security.oauth2.resourceserver.jwt.jwk-set-uri}")
    private String jwkSetUri;

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity httpSecurity)
        throws Exception {
        JwtAuthenticationConverter jwtAuthenticationConverter =
            new JwtAuthenticationConverter();

        jwtAuthenticationConverter.setJwtGrantedAuthoritiesConverter(
            new KeycloakJwtRoleConverter()
        );

        return httpSecurity
            .csrf(csrf -> csrf.disable())
            .sessionManagement(session ->
                session.sessionCreationPolicy(SessionCreationPolicy.STATELESS)
            )
            .authorizeHttpRequests(authorizeRequests ->
                authorizeRequests

                    // Preflight CORS
                    .requestMatchers(HttpMethod.OPTIONS, "/**")
                    .permitAll()

                    // Booking: usuario autenticado normal o admin
                    .requestMatchers(HttpMethod.POST, "/api/v1/booking")
                    .hasAnyRole("USER", "ADMIN")

                    // Eventos: lectura para usuarios normales y admin
                    .requestMatchers(HttpMethod.GET, "/api/v1/inventory/events")
                    .hasAnyRole("USER", "ADMIN")

                    .requestMatchers(
                        HttpMethod.GET,
                        "/api/v1/inventory/event/*"
                    )
                    .hasAnyRole("USER", "ADMIN")

                    // Inventario / sedes: solo admin
                    .requestMatchers("/api/v1/inventory/venues")
                    .hasRole("ADMIN")

                    .requestMatchers("/api/v1/inventory/venue/*")
                    .hasRole("ADMIN")

                    .requestMatchers(
                        HttpMethod.PUT,
                        "/api/v1/inventory/venue/*"
                    )
                    .hasRole("ADMIN")

                    .requestMatchers(
                        HttpMethod.POST,
                        "/api/v1/inventory/events"
                    )
                    .hasRole("ADMIN")

                    .requestMatchers(
                        HttpMethod.DELETE,
                        "/api/v1/inventory/event/*"
                    )
                    .hasRole("ADMIN")

                    .requestMatchers("/api/v1/inventory/event/*/capacity/*")
                    .hasRole("ADMIN")

                    .requestMatchers(
                        "/api/v1/inventory/event/*/capacity/release/*"
                    )
                    .hasRole("ADMIN")

                    // Simulación: por ahora usuario o admin.
                    // Si luego decides que también es admin-only, cambia esto.
                    .requestMatchers(
                        "/api/v1/inventory/event/*/simulate-concurrent-booking"
                    )
                    .hasAnyRole("USER", "ADMIN")

                    // Cualquier otro endpoint requiere login
                    .anyRequest()
                    .authenticated()
            )
            .oauth2ResourceServer(oauth ->
                oauth.jwt(jwt ->
                    jwt.jwtAuthenticationConverter(jwtAuthenticationConverter)
                )
            )
            .build();
    }

    @Bean
    public JwtDecoder jwtDecoder() {
        return NimbusJwtDecoder.withJwkSetUri(jwkSetUri).build();
    }
}
