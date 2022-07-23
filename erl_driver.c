/* port_driver.c */
#include "erl_driver.h"
#include <string.h>

#define DRV_NAME "libsum"

//import rust fn
extern char* port_call(char*, size_t);

typedef struct {
    ErlDrvPort port;
} erl_data;


static ErlDrvData port_drv_start(ErlDrvPort port, char *buff)
{
    erl_data* d = (erl_data*)driver_alloc(sizeof(erl_data));
    d->port = port;
    return (ErlDrvData)d;
}

static void port_drv_stop(ErlDrvData handle)
{
    driver_free((char*)handle);
}

static void port_drv_output(ErlDrvData handle, char *buff,
			       ErlDrvSizeT bufflen)
{
    erl_data* d = (erl_data*)handle;

    char* res = port_call(buff, bufflen);

    driver_output(d->port, res, strlen(res));

}

ErlDrvEntry driver_entry = {
    NULL,			/* F_PTR init, called when driver is loaded */
    port_drv_start,		/* L_PTR start, called when port is opened */
    port_drv_stop,		/* F_PTR stop, called when port is closed */
    port_drv_output,		/* F_PTR output, called when erlang has sent */
    NULL,			/* F_PTR ready_input, called when input descriptor ready */
    NULL,			/* F_PTR ready_output, called when output descriptor ready */
    DRV_NAME,		/* char *driver_name, the argument to open_port */
    NULL,			/* F_PTR finish, called when unloaded */
    NULL,                       /* void *handle, Reserved by VM */
    NULL,			/* F_PTR control, port_command callback */
    NULL,			/* F_PTR timeout, reserved */
    NULL,			/* F_PTR outputv, reserved */
    NULL,                       /* F_PTR ready_async, only for async drivers */
    NULL,                       /* F_PTR flush, called when port is about
				   to be closed, but there is data in driver
				   queue */
    NULL,                       /* F_PTR call, much like control, sync call
				   to driver */
    NULL,                       /* unused */
    ERL_DRV_EXTENDED_MARKER,    /* int extended marker, Should always be
				   set to indicate driver versioning */
    ERL_DRV_EXTENDED_MAJOR_VERSION, /* int major_version, should always be
				       set to this value */
    ERL_DRV_EXTENDED_MINOR_VERSION, /* int minor_version, should always be
				       set to this value */
    0,                          /* int driver_flags, see documentation */
    NULL,                       /* void *handle2, reserved for VM use */
    NULL,                       /* F_PTR process_exit, called when a
				   monitored process dies */
    NULL                        /* F_PTR stop_select, called to close an
				   event object */
};

DRIVER_INIT(DRV_NAME) /* must match name in driver_entry */
{
    return &driver_entry;
}
