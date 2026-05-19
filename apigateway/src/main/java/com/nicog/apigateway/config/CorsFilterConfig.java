package com.nicog.apigateway.config;

import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import java.io.IOException;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.Ordered;
import org.springframework.core.annotation.Order;
import org.springframework.web.filter.OncePerRequestFilter;

@Configuration
@Order(Ordered.HIGHEST_PRECEDENCE)
public class CorsFilterConfig extends OncePerRequestFilter {

    @Override
    protected void doFilterInternal(
        HttpServletRequest request,
        HttpServletResponse response,
        FilterChain filterChain
    ) throws ServletException, IOException {
        response.setHeader(
            "Access-Control-Allow-Origin",
            "http://localhost:5173"
        );
        response.setHeader(
            "Access-Control-Allow-Methods",
            "GET,POST,PUT,DELETE,OPTIONS"
        );
        response.setHeader(
            "Access-Control-Allow-Headers",
            "Authorization,Content-Type,Accept"
        );
        response.setHeader("Access-Control-Allow-Credentials", "true");
        response.setHeader("Access-Control-Max-Age", "3600");

        if ("OPTIONS".equalsIgnoreCase(request.getMethod())) {
            response.setStatus(HttpServletResponse.SC_OK);
            return;
        }

        filterChain.doFilter(request, response);
    }
}
