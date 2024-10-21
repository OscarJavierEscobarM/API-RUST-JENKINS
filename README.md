Descargar y correr la imagen Docker Jenkins

Descargar la imagen Docker Jenkins

![image](https://github.com/user-attachments/assets/8b3e4983-2e76-4114-980d-d5ee3675df68)

Inicializar el Docker container de Jenkins

docker run --rm -u root -p 8080:8080 -v jenkins-data:/var/jenkins_home -v $(which docker):/usr/bin/docker -v /var/run/docker.sock:/var/run/docker.sock -v "$HOME":/home --name jenkins_server jenkins/jenkins:lts

Step 2: Start the Jenkins Docker container.
Enter the following command on one line. You may need to copy it to a text editor if you are viewing a PDF
version of this lab to avoid line breaks. This command will start the Jenkins Docker container and then allow
Docker commands to be executed inside your Jenkins server.
devasc@labvm:~/labs/devnet-src/jenkins/sample-app$ docker run --rm -u root -p
8080:8080 -v jenkins-data:/var/jenkins_home -v $(which
docker):/usr/bin/docker -v /var/run/docker.sock:/var/run/docker.sock -v
"$HOME":/home --name jenkins_server jenkins/jenkins:lts
The options used in this docker run command are as follows:
o --rm - This option automatically removes the Docker container when you stop running it.
o -u - This option specifies the user. You want this Docker container to run as root so that all Docker
commands entered inside the Jenkins server are allowed.
o -p - This option specifies the port the Jenkins server will run on locally.
o -v - These options bind mount volumes needed for Jenkins and Docker. The first -v specifies where
Jenkins data will be stored. The second -v specifies where to get Docker so that you can run Docker
inside the Docker container that is running the Jenkins server. The third -v specifies the PATH
variable for the home directory.
Step 3: Verify the Jenkins server is running.
The Jenkins server should now be running. Copy the admin password that displays in the output, as shown in
the following.
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 12 of 26 www.netacad.com
Do not enter any commands in this server window. If you accidentally stop the Jenkins server, you will need
to re-enter the docker run command from Step 2 above. After the initial install, the admin password is
displayed as shown below.
<output omitted>
*************************************************************
*************************************************************
*************************************************************
Jenkins initial setup is required. An admin user has been created and a password
generated.
Please use the following password to proceed to installation:
77dc402e31324c1b917f230af7bfebf2 <--Your password will be different
This may also be found at: /var/jenkins_home/secrets/initialAdminPassword
*************************************************************
*************************************************************
*************************************************************
<output omitted>
2020-05-12 16:34:29.608+0000 [id=19] INFO hudson.WebAppMain$3#run: Jenkins is
fully up and running
Note: If you lose the password, or it does not display as shown above, or you need to restart the Jenkins
sever, you can always retrieve the password by accessing the command line of Jenkins Docker container.
Create a second terminal window in VS Code and enter the following commands so that you do not stop the
Jenkins server.:
devasc@labvm:~/labs/devnet-src/jenkins/sample-app$ docker exec -it
jenkins_server /bin/bash
root@19d2a847a54e:/# cat /var/jenkins_home/secrets/initialAdminPassword
77dc402e31324c1b917f230af7bfebf2
root@19d2a847a54e:/# exit
exit
devasc@labvm:~/labs/devnet-src/jenkins/sample-app$
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 13 of 26 www.netacad.com
Note: Your container ID (19d2a847a54e highlighted above) and password will be different.
Step 4: Investigate the levels of abstraction currently running on your computer.
The following ASCII diagram shows the levels of abstraction in this Docker-inside-Docker (dind)
implementation. This level of complexity is not unusual in today’s networks and cloud infrastructures.
+----------------------------------------+
|Your Computer’s Operating System |
| +----------------------------------+ |
| |DEVASC VM | |
| | +----------------------------+ | |
| | |Docker container | | |
| | | +----------------------+ | | |
| | | | Jenkins server | | | |
| | | | +----------------+ | | | |
| | | | |Docker container| | | | |
| | | | +----------------+ | | | |
| | | +----------------------+ | | |
| | +----------------------------+ | |
| +----------------------------------+ |
+----------------------------------------+
Part 5: Configure Jenkins
In this Part, you will complete the initial configuration of the Jenkins server.
Step 1: Open a web browser tab.
Navigate to http://localhost:8080/ and login in with your copied admin password.
Step 2: Install the recommended Jenkins plugins.
Click Install suggested plugins and wait for Jenkins to download and install the plugins. In the terminal
window, you will see log messages as the installation proceeds. Be sure that you do not close this terminal
window. You can open another terminal window for access to the command line.
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 14 of 26 www.netacad.com
Step 3: Skip creating a new admin user.
After the installation finishes, you are presented with the Create First Admin User window. For now, click
Skip and continue as admin at the bottom.
Step 4: Skip creating an instance configuration.
In the Instance Configuration window, do not change anything. Click Save and Finish at the bottom.
Step 5: Start using Jenkins.
In the next window, click Start using Jenkins. You should now be on the main dashboard with a Welcome
to Jenkins! message.
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 15 of 26 www.netacad.com
Part 6: Use Jenkins to Run a Build of Your App
The fundamental unit of Jenkins is the job (also known as a project). You can create jobs that do a variety of
tasks including the following:
o Retrieve code from a source code management repository such as GitHub.
o Build an application using a script or build tool.
o Package an application and run it on a server
In this part, you will create a simple Jenkins job that retrieves the latest version of your sample-app from
GitHub and runs the build script. In Jenkins, you can then test your app (Part 7) and add it to a development
pipeline (Part 8).
Step 1: Create a new job.
a. Click the Create a job link directly below the Welcome to Jenkins! message. Alternatively, you can click
New Item in the menu on the left.
b. In the Enter an item name field, fill in the name BuildAppJob.
c. Click Freestyle project as the job type. In the description, the SCM abbreviation stands for software
configuration management, which is a classification of software that is responsible for tracking and
controlling changes in software.
d. Scroll to the bottom and click OK.
Step 2: Configure the Jenkins BuildAppJob.
You are now in the configuration window where you can enter details about your job. The tabs across the top
are just shortcuts to the sections below. Click through the tabs to explore the options you can configure. For
this simple job, you only need to add a few configuration details.
a. Click the General tab, add a description for your job. For example, "My first Jenkins job."
b. Click the Source Code Management tab and choose the Git radio button. In the Repository URL field,
add your GitHub repository link for the sample-app taking care to enter your case-sensitive username. Be
sure to add the .git extension at the end of your URL. For example:
https://github.com/github-username/sample-app.git
c. For Credentials, click the Add button and choose Jenkins.
d. In the Add Credentials dialog box, fill in your GitHub username and password, and then click Add.
Note: You will receive an error message that the connection has failed. This is because you have not
selected the credentials yet.
e. In the dropdown for Credentials where it currently says None, choose the credentials you just
configured.
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 16 of 26 www.netacad.com
f. After you have added the correct URL and credentials, Jenkins tests access to the repository. You should
have no error messages. If you do, verify your URL and credentials. You will need to Add them again as
there is no way at this point to delete the ones you previously entered.
g. At the top of the BuildAppJob configuration window, click the Build tab.
h. For the Add build step dropdown, choose Execute shell.
i. In the Command field, enter the command you use to run the build for sample-app.sh script.
bash ./sample-app.sh
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 17 of 26 www.netacad.com
j. Click the Save button. You are returned to the Jenkins dashboard with the BuildAppJob selected.
Step 3: Have Jenkins build the app.
On the left side, click Build Now to start the job. Jenkins will download your Git repository and execute the
build command bash ./sample-app.sh. Your build should succeed because you have not changed anything
in the code since Part 3 when you modified the code.
Step 4: Access the build details.
On the left, in the Build History section, click your build number which should be the #1 unless you have built
the app multiple times.
Step 5: View the console output.
On the left, click Console Output. You should see output similar to the following. Notice the success
messages at the bottom as well as the output from the docker ps -a command. Two docker containers are
running: one for your sample-app running on local port 5050 and one for Jenkins on local port 8080.
Started by user admin
Running as SYSTEM
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 18 of 26 www.netacad.com
Building in workspace /var/jenkins_home/workspace/BuildAppJob
using credential 0cf684ea-48a1-4e8b-ba24-b2fa1c5aa3df
Cloning the remote Git repository
Cloning repository https://github.com/github-user/sample-app
> git init /var/jenkins_home/workspace/BuildAppJob # timeout=10
Fetching upstream changes from https://github.com/github-user/sample-app
> git --version # timeout=10
using GIT_ASKPASS to set credentials
> git fetch --tags --progress -- https://github.com/github-user/sample-app
+refs/heads/*:refs/remotes/origin/* # timeout=10
> git config remote.origin.url https://github.com/github-user/sample-app # timeout=10
> git config --add remote.origin.fetch +refs/heads/*:refs/remotes/origin/* #
timeout=10
> git config remote.origin.url https://github.com/github-user/sample-app # timeout=10
Fetching upstream changes from https://github.com/github-user/sample-app
using GIT_ASKPASS to set credentials
> git fetch --tags --progress -- https://github.com/github-user/sample-app
+refs/heads/*:refs/remotes/origin/* # timeout=10
> git rev-parse refs/remotes/origin/master^{commit} # timeout=10
> git rev-parse refs/remotes/origin/origin/master^{commit} # timeout=10
Checking out Revision 230ca953ce83b5d6bdb8f99f11829e3a963028bf
(refs/remotes/origin/master)
> git config core.sparsecheckout # timeout=10
> git checkout -f 230ca953ce83b5d6bdb8f99f11829e3a963028bf # timeout=10
Commit message: "Changed port numbers from 8080 to 5050"
> git rev-list --no-walk 230ca953ce83b5d6bdb8f99f11829e3a963028bf # timeout=10
[BuildAppJob] $ /bin/sh -xe /tmp/jenkins1084219378602319752.sh
+ bash ./sample-app.sh
Sending build context to Docker daemon 6.144kB
Step 1/7 : FROM python
---> 4f7cd4269fa9
Step 2/7 : RUN pip install flask
---> Using cache
---> 57a74c0dff93
Step 3/7 : COPY ./static /home/myapp/static/
---> Using cache
---> aee4eb712490
Step 4/7 : COPY ./templates /home/myapp/templates/
---> Using cache
---> 594cdc822490
Step 5/7 : COPY sample_app.py /home/myapp/
---> Using cache
---> a001df90cf0c
Step 6/7 : EXPOSE 5050
---> Using cache
---> eae896e0a98c
Step 7/7 : CMD python3 /home/myapp/sample_app.py
---> Using cache
---> 272c61fddb45
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 19 of 26 www.netacad.com
Successfully built 272c61fddb45
Successfully tagged sampleapp:latest
9c8594e62079c069baf9a88a75c13c8c55a3aeaddde6fd6ef54010953c2d3fbb
CONTAINER ID IMAGE COMMAND CREATED
STATUS PORTS NAMES
9c8594e62079 sampleapp "/bin/sh -c 'python …" Less than a second
ago Up Less than a second 0.0.0.0:5050->5050/tcp samplerunning
e25f233f9363 jenkins/jenkins:lts "/sbin/tini -- /usr/…" 29 minutes ago
Up 29 minutes 0.0.0.0:8080->8080/tcp, 50000/tcp jenkins_server
Finished: SUCCESS
Step 6: Open another web browser tab and verify sample app is running.
Type in the local address, localhost:5050. You should see the content of your index.html displayed in light
steel blue background color with You are calling me from 172.17.0.1 displayed in as H1.
Part 7: Use Jenkins to Test a Build
In this part, you will create a second job that tests the build to ensure that it is working properly.
Note: You need to stop and remove the samplerunning docker container.
devasc@labvm:~/labs/devnet-src/jenkins/sample-app$ docker stop samplerunning
samplerunning
devasc@labvm:~/labs/devnet-src/jenkins/sample-app$ docker rm samplerunning
samplerunning
Step 1: Start a new job for testing your sample-app.
a. Return to the Jenkins web browser tab and click the Jenkins link in the top left corner to return to the
main dashboard.
b. Click the New Item link to create a new job.
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 20 of 26 www.netacad.com
c. In the Enter an item name field, fill in the name TestAppJob.
d. Click Freestyle project as the job type.
e. Scroll to the bottom and click OK.
Step 2: Configure the Jenkins TestAppJob.
a. Add a description for your job. For example, "My first Jenkins test."
b. Leave Source Code Management set to None.
c. Click the Build Triggers tab and check the box, Build after other projects are built. For Projects to
watch, fill in the name BuildAppJob.
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 21 of 26 www.netacad.com
Step 3: Write the test script that should run after a stable build of the BuildAppJob.
a. Click the Build tab.
b. Click Add build step and choose Execute shell.
c. Enter the following script. The if command should be all on one line including the ; then. This command
will grep the output returned from the cURL command to see if You are calling me from 172.17.0.1 is
returned. If true, the script exits with a code of 0, which means that there are no errors in the
BuildAppJob build. If false, the script exits with a code of 1 which means the BuildAppJob failed.
if curl http://172.17.0.1:5050/ | grep "You are calling me from 172.17.0.1"; then
 exit 0
else
 exit 1
fi
d. Click Save and then the Back to Dashboard link on the left side.
Step 4: Have Jenkins run the BuildAppJob job again.
a. Refresh the web page with the refresh button for your browser.
b. You should now see your two jobs listed in a table. For the BuildAppJob job, click the build button on the
far right (a clock with an arrow).
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 22 of 26 www.netacad.com
Step 5: Verify both jobs completed.
If all goes well, you should see the timestamp for the Last Success column update for both BuildAppJob
and TestAppJob. This means your code for both jobs ran without error. But you can also verify this for
yourself.
Note: If timestamps do not update, make sure enable auto refresh is turned on by clicking the link in the top
right corner.
a. Click the Link for TestAppJob. Under Permalinks, click the link for your last build, and then click
Console Output. You should see output similar to the following:
Started by upstream project "BuildAppJob" build number 13
originally caused by:
Started by user admin
Running as SYSTEM
Building in workspace /var/jenkins_home/workspace/TestAppJob
[TestAppJob] $ /bin/sh -xe /tmp/jenkins1658055689664198619.sh
+ grep You are calling me from 172.17.0.1
+ curl http://172.17.0.1:5050/
 % Total % Received % Xferd Average Speed Time Time Time Current
 Dload Upload Total Spent Left Speed
 0 0 0 0 0 0 0 0 --:--:-- --:--:-- --:--:-- 0
100 177 100 177 0 0 29772 0 --:--:-- --:--:-- --:--:-- 35400
 <h1>You are calling me from 172.17.0.1</h1>
+ exit 0
Finished: SUCCESS
Lab - Build a CI/CD Pipeline Using Jenkins
© 2020 - 2024 Cisco and/or its affiliates. All rights reserved. Cisco Public Page 23 of 26 www.netacad.com
b. It is not necessary to verify your sample app is running because the TestAppJob already did this for you.
However, you can open a browser tab for 172.17.0.1:5050 to see that it is indeed running.
