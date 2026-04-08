CREATE TABLE sede(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    direccion VARCHAR(255),
    capacidad_total BIGINT NOT NULL
);

CREATE TABLE evento(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    sede_id BIGINT NOT NULL,
    capacidad_total BIGINT NOT NULL,
    capacidad_restante BIGINT NOT NULL,
    CONSTRAINT fk_evento_sede
        FOREIGN KEY (sede_id) REFERENCES sede(id) ON DELETE CASCADE
);
