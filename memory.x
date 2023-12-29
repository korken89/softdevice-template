/* 
Softdevice specification, update ORIGIN and LENGTH for your application.

Note that the RAM is set to 0 size. If you need RAM, make sure it is NOT overlapping with application RAM.
*/

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
    SOFTDEVICE : ORIGIN = addr_as_above, LENGTH = length_as_above
    ...
}

__softdevice = ORIGIN(SOFTDEVICE);

*/
