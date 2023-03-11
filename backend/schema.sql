DROP TABLE IF EXISTS Locations;
CREATE TABLE Locations (id INT, name TEXT, url TEXT, lat FLOAT, long FLOAT PRIMARY KEY (`id`));
INSERT INTO Customers (id, name, url,lat,long) VALUES (1, 'climbing1', 'wwww.test1', -122, 45), (2, 'climbinging 2', 'www.test2', -123, 46) ;