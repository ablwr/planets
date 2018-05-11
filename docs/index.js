var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");

function setup(){
  canvas.setAttribute('width', window.innerWidth);
  canvas.setAttribute('height', window.innerHeight);
  ctx.translate(canvas.width/2,canvas.height/2);
}
setup();

function draw(planets_data) {
  for (var i=0; i<planets_data.length; i++) {
    drawPlanet(planets_data[i].lat,planets_data[i].long,planets_data[i].rad, planets_data[i].name);
  }
}

function drawEarth() {
  var img = document.getElementById('Earth');
  ctx.drawImage(img, 0, 0, 36, 36);
}

function drawSun() {
  var img = document.getElementById('Sun');
  ctx.drawImage(img, 0, 0, 36, 36);
}

function drawPlanet(lat, long, rad, name) {
  if (name == "Neptune" || name == "Uranus") {
    var x = rad*Math.cos(long)*Math.cos(lat) * 11
    var y = -(rad*Math.cos(lat)*Math.sin(long) * 11)
  } else if (name == "Saturn") {
    var x = rad*Math.cos(long)*Math.cos(lat) * 25
    var y = -(rad*Math.cos(lat)*Math.sin(long) * 25)    
  } else if (name == "Jupiter") {
    var x = rad*Math.cos(long)*Math.cos(lat) * 40
    var y = -(rad*Math.cos(lat)*Math.sin(long) * 40)    
  } else {
    var x = rad*Math.cos(long)*Math.cos(lat) * 100
    var y = -(rad*Math.cos(lat)*Math.sin(long) * 100)
  }
  var img = document.getElementById(name);
  ctx.beginPath();
  ctx.drawImage(img, x, y, 26, 26);
  ctx.font = '12px sans-serif';
  ctx.fillStyle = '#FFFFFF';
  ctx.fillText(name, x, y);
  ctx.closePath();
  ctx.fill();
}

