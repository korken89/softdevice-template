/* Softdevice specification, update ORIGIN and LENGTH for your application */

MEMORY
{
    FLASH :       ORIGIN = 0x00000000, LENGTH = 16K
    RAM   :       ORIGIN = 0x20000000, LENGTH = 0
}

/*

Do this in your application's linker script to define the softdevice:

MEMORY
{
    ...
    SOFTDEVICE : ORIGIN = addr, LENGTH = same_as_above
    ...
}

__softdevice = ORIGIN(SOFTDEVICE);

*/
