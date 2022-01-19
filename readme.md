# MXG | V.0.1 - Currently in active development

MXG is a debuger printing library for the Rust language.

## Pre-requsites 

* Rust 2021 or later
* Rust 1.58 or later

## Mini-Book

Quick overview of the main points of the library.

### Kind

The Messages are classified by four categories: 

Cat. | Colour* | Descriptions
---- | ------- | ------------
Error | Red |
Warning | Yellow | 
Info | White |
Valid | Green

*The library use the ansi_term crate for managing its colours printed.

### DebugMessage

Once printed, a DebugMessage will look like this.


```bash
<FileName> <Line> - <Col> | <Reason>

    l.<Line> | <Code>
    <Subline>

   <hint>
```

With a more concise exemple, it will look this.

```bash
up.sql 1 - 36 | Invalid end of query

    l.1 | SELECT * FROM table;;
                              -

   Consider removing the last character.
```

#### Subline

The sub-line is the line underneath the code where the nature of the modifications will be shown. The End-User it has no a direct access, but through multiple methods that will also highlights the portion of code concerned.

method | colour | symbol | desc
------ | ------ | ------ | ----
add | Green | + | Will show what to add
remove | Red | - | Will show what to remove
highlight | Yellow | ^ | Will highlight the portion of code that needs modification.


#### How to consume it?

Two possibilities: One with the Buffer struct, the other is by calling the consume() method in the Message Trait.

### Buffer

The Buffer is the struct with which you'll collect the Messages. The advantage of collecting them with this struct, is that you can impose a limit upon the number of errors found before causing an early return, and to not worry about consuming the Messages, as it will do it on its own when being Dropped.

### InfoMessage - Currently unstable and experimental

Similar to the DebugMessage, it differs as it does not show anycode and that is works currently out of the normal library workflow - It cannot be added technically to the Buffer.

Two ways to consume it: call the method consume(), or drop it.

