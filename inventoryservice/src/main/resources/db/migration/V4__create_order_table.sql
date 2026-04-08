CREATE TABLE orden(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    precio_total DECIMAL(10, 2) NOT NULL,
    recuento_tickets BIGINT NOT NULL,
    placed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    id_cliente BIGINT,
    id_evento BIGINT,
    CONSTRAINT fk_orden_cliente
        FOREIGN KEY (id_cliente) REFERENCES cliente(id) ON DELETE SET NULL,
    CONSTRAINT fk_orden_evento
        FOREIGN KEY (id_evento) REFERENCES evento(id) ON DELETE SET NULL
);
