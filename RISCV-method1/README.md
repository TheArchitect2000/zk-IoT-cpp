### About the main folder, it should be compiled in Arduino 

Steps to Compile the Project:

1. Open the Main Folder in Arduino IDE
•	Download or clone the repository, ensuring you have the necessary files on your local machine.
•	Launch Arduino IDE.
•	Open the main folder of the project using Arduino by navigating to: File → Open → Select the .ino file located in the main folder of your project.

2. Select the Correct Board in Arduino IDE
•	Locate the board area on the left sidebar (Arduino IDE 2.x) or go to Tools → Board in Arduino IDE 1.x.
•	Select the correct board: From the dropdown list, choose ESP32C3 Dev Module.

3. Install Required Libraries
•	To ensure the project compiles correctly, you need to install specific libraries from the Arduino Library Manager.
•	Go to Tools → Manage Libraries....
In the Library Manager: Search for and install the following library: ArduinoJson by Benoit Blanchon, version 7.2.0 (make sure to install the correct version).

4. Compile and Upload
