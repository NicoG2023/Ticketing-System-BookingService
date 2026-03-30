package com.nicog.inventoryservice.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
//El uso de Lombok me evita tener que escribir los getters y setters, asi como tambien constructores
//Es decir, me evita el tener que escribir codigo repetitivo, es usable en produccion
//el unico pero seria que no se puede controlar los Setter, ya que actualmente se permite modificar TODO libremente
//esto quiere decir que se podrian exponer setters que no deberia
@Entity
@Getter
@Setter
@AllArgsConstructor
//El AllArgsConstructor no es necesario para JPA casi nunca, puede generar constructores que no respetan invariantes
@NoArgsConstructor
@Table(name = "venue")
public class Venue {

    @Id
    @Column(name = "id")
    private Long id;

    @Column(name = "name", nullable = false)
    private String name;

    @Column(name = "address", length = 250)
    private String address;

    @Column(name = "total_capacity")
    private Long totalCapacity;

    //Se podria agregar otro atributo events, que marque una relacion bidireccional con Event, a traves de un List
    //Para esto se le agrega la anotacion @OneToMany, aunque no siempre es conveniente, depende del caso
    //por ejemplo, un Venue puede tener muchos eventos, y se necesita ver todos los eventos de un venue, o tambien
    //crear eventos dentro de un venue, o validar reglas del venue sobre sus eventos.
}
