Descargar y correr la imagen Docker Jenkins

Descargar la imagen Docker Jenkins

![image](https://github.com/user-attachments/assets/1cd58467-e979-44d3-80aa-f1b53d02a1db)

Inicializar el Docker container de Jenkins

docker run --rm -u root -p 8080:8080 -v jenkins-data:/var/jenkins_home -v $(which docker):/usr/bin/docker -v /var/run/docker.sock:/var/run/docker.sock -v "$HOME":/home --name jenkins_server jenkins/jenkins:lts

obtenemos la siguiente password que sera de utilidad para los siguientes pasos y observamos que jenkins ya esta corriendo
![image](https://github.com/user-attachments/assets/8ec79781-ba3c-4b57-90c5-170f7b842839)

ahora configuraremos Jenkins, primero tendremos que entrar al navegadorcon la URL "http://localhost:8080/" e ingresaremos la contraseña obtenida en el paso anterior

![image](https://github.com/user-attachments/assets/90503b61-d9cf-40bd-bff8-306694702a62)

instalamos los Jenkins pliguins recomendados.

![image](https://github.com/user-attachments/assets/082691aa-431c-4b59-be52-72bfe56a2570)
![image](https://github.com/user-attachments/assets/98d12d37-1e3d-4024-ae05-28eec4491d52)

Creamos un nuevo usuario administrador y skipeamos la creacion de una instancia

![image](https://github.com/user-attachments/assets/e02bf518-fea2-4822-a26f-4bfca6824b59)
![image](https://github.com/user-attachments/assets/4efbc09c-c861-4146-9c5f-53aed0157253)

ahora usaremos Jenkins para construir y correr nuesta App

primero crearemos nuestro primer trabajo y configuraremos las credenciales para conectarlo a nuestro proyecto de GitHub

![image](https://github.com/user-attachments/assets/57dc9dc9-b7f3-4068-8813-9eb79d02fd3c)
![image](https://github.com/user-attachments/assets/c33983ec-0493-444f-a86a-b28c98689072)
![image](https://github.com/user-attachments/assets/b4b44a46-290f-4249-8c76-fcfd30874d3f)
![image](https://github.com/user-attachments/assets/fd514b1b-9b6b-4f2f-ba31-bc4dbb5a454f)

agregamos el siguiente comando para que cree la imagen de la app y el docker lo construya y ejecute

![image](https://github.com/user-attachments/assets/6368e6bb-5016-4be7-a129-095b9c20c9bc)

Despues de contruir nuestra aplicacion comenzara realizar los procesos indicados en api-rust.sh y al final nos mostrara lo siguiente

![image](https://github.com/user-attachments/assets/f1f34331-7a10-45da-8316-d17eab8eeaec)
![image](https://github.com/user-attachments/assets/8d1292fd-8e7f-412f-806a-70a8d53b7e08)

para verificarlo, abriremos otra pestaña en el navegador con la URL al localhost:5050/actividad

![image](https://github.com/user-attachments/assets/6343f4ca-4ebf-493e-9f20-fc3c503efeaa)

ahora llenaremos algunos datos utilizando Postman y posteriormente le haremos algunas pruebas utilizando Jenkins

![image](https://github.com/user-attachments/assets/e9e5aa40-8e90-4bcb-a4d7-5918c9585087)
![image](https://github.com/user-attachments/assets/79bff267-eefa-4251-81ac-264862f7e417)

Comprobamos:

![image](https://github.com/user-attachments/assets/ec01addd-94bc-4db9-aa80-d6bc6786d8a7)

ahora creamos un projecto que estara basado en nuestra API y ejecutara una serie de codigos para realizar pruebas a nuestra API

![image](https://github.com/user-attachments/assets/7506f936-885c-4606-b14b-65afcb83f134)
![image](https://github.com/user-attachments/assets/ead8a660-3268-438a-9f6d-a51741624a01)

realizaremos las pruebas con las siguientes lineas de codigo:

if curl http://172.17.0.1:5050/actividad; then
	 
   exit 0

else

   exit 1

fi

![image](https://github.com/user-attachments/assets/5890a38c-4128-42bf-9aab-ab02f40fe489)

despues de ejecutar nuesto proyecto de prueba se revisarán los detalles de la construcion, observando que ha sido exitosa

![image](https://github.com/user-attachments/assets/658c8db2-0344-403e-9e06-ac777da419ba)
![image](https://github.com/user-attachments/assets/b6bfa391-4f40-4af4-aed2-000b60733334)


