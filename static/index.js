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
  ctx.beginPath();
  ctx.font = '36px sans-serif';
  ctx.fillText("🌞", 0, 0);
  ctx.closePath();
  ctx.fill();
}

function drawPlanet(lat, long, rad, name) {
  ctx.beginPath();
  ctx.font = '14px sans-serif';
  var x = rad*Math.cos(long)*Math.cos(lat)
  var y = rad*Math.cos(lat)*Math.sin(long)
  ctx.fillStyle = '#FFFFFF';
  console.log("x is " + x + ", y is " + y)
  ctx.fillText("⭐️" + name, x * 24, -y * 24);
  ctx.closePath();
  ctx.fill();
}

