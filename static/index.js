var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");

function setup(){
  canvas.setAttribute('width', window.innerWidth);
  canvas.setAttribute('height', window.innerHeight);
  ctx.translate(canvas.width/2,canvas.height/2);
}
setup();

function draw(planets_data) {
  drawSun();
  
  for (var i=0; i<planets_data.length; i++) {
    drawPlanet(planets_data[i].lat,planets_data[i].long,planets_data[i].rad, planets_data[i].name);
  }
  
}

function drawSun() {
  img = document.getElementById('Sun');
  ctx.drawImage(img, 0, 0, 40, 40);
}

function drawPlanet(lat, long, rad, name) {
  var x = rad*Math.cos(long)*Math.cos(lat) * 20
  var y = -(rad*Math.cos(lat)*Math.sin(long) * 20)
  img = document.getElementById(name);
  ctx.drawImage(img, x, y, 30, 30);
  ctx.beginPath();
  ctx.font = '12px sans-serif';
  ctx.fillStyle = '#FFFFFF';
  ctx.fillText(name, x, y);
  ctx.closePath();
  ctx.fill();
}

