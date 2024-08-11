# Especifica el archivo del módulo
obj-m += sysinfo.o

# Regla para construir el módulo
all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

# Regla para limpiar archivos generados
clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
