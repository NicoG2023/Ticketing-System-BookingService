package com.nicog.apigateway.config;

import java.util.Collection;
import java.util.Collections;
import java.util.Map;
import java.util.stream.Collectors;
import org.springframework.core.convert.converter.Converter;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.oauth2.jwt.Jwt;

public class KeycloakJwtRoleConverter
    implements Converter<Jwt, Collection<GrantedAuthority>>
{

    private static final String CLIENT_ID = "ticket-frontend";

    @Override
    public Collection<GrantedAuthority> convert(Jwt jwt) {
        Map<String, Object> resourceAccess = jwt.getClaim("resource_access");

        if (resourceAccess == null || !resourceAccess.containsKey(CLIENT_ID)) {
            return Collections.emptyList();
        }

        Object clientAccessObject = resourceAccess.get(CLIENT_ID);

        if (!(clientAccessObject instanceof Map<?, ?> clientAccess)) {
            return Collections.emptyList();
        }

        Object rolesObject = clientAccess.get("roles");

        if (!(rolesObject instanceof Collection<?> roles)) {
            return Collections.emptyList();
        }

        return roles
            .stream()
            .filter(String.class::isInstance)
            .map(String.class::cast)
            .map(String::toUpperCase)
            .map(role -> new SimpleGrantedAuthority("ROLE_" + role))
            .collect(Collectors.toSet());
    }
}
