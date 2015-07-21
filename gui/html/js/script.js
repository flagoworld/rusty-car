var loadTimer;
var loadDots = '';

$(function()
{
    loadUI();

    loadTimer = setInterval(function()
    {
        loadDots += '.';

        if(loadDots.length > 3)
        {
            loadDots = '';
        }

        $('#loading h1').text('INITIALIZING' + loadDots);
    }, 200);
});

$(window).load(function()
{
    setTimeout(function()
    {
        $('#loading').fadeOut(function()
        {
            clearInterval(loadTimer);
        });
    }, 400);
});

function loadUI()
{
    loadDashboardSpeedo();
    loadDashboardRPM();
    loadDashboardHeater();
    loadDashboardAudio();

    loadDashboardLabels();

    loadWebSocket();
}

var gauge_mph;
function loadDashboardSpeedo()
{
    var gauge = new Gauge(
    {
    	renderTo    : 'dashboard-ecu-speedo',
    	width       : 400,
    	height      : 400,
    	glow        : true,
    	units       : 'MPH',
    	title       : false,
    	minValue    : 0,
    	maxValue    : 140,
        valueFormat:
        {
            "int": 3,
            dec: 0
        },
    	majorTicks  : ['0','10','20','30','40','50','60','70','80','90','100','110','120','130','140'],
    	minorTicks  : 2,
    	strokeTicks : false,
    	/*highlights  : [
    		{ from : 0,   to : 100, color : 'rgba(0,   255, 0, .15)' },
    		{ from : 100, to : 160, color : 'rgba(255, 255, 0, .15)' },
    		{ from : 160, to : 220, color : 'rgba(255, 30,  0, .25)' }
    	],*/
    	highlights : false,
    	colors      :
        {
    		plate      : '#222',
    		majorTicks : '#f5f5f5',
    		minorTicks : '#ddd',
    		title      : '#fff',
    		units      : '#ccc',
    		numbers    : '#eee',
    		needle     : { start : 'rgba(240, 240, 240, 1)', end : 'rgba(255, 255, 255, 1)' }
    	}
    });

    gauge.onready = function()
    {
        // var val = 0;
    	// setInterval(function()
        // {
        //     val += 1;
        //
        //     if(val > 60) val = 0;
        //
    	// 	gauge.setValue(val);
    	// }, 100);
    };

    gauge.draw();

    gauge_mph = gauge;
}

var gauge_rpm;
function loadDashboardRPM()
{
    var gauge = new Gauge(
    {
    	renderTo    : 'dashboard-ecu-rpm',
    	width       : 200,
    	height      : 200,
    	glow        : true,
    	units       : 'RPMx1000',
    	title       : false,
    	minValue    : 0,
    	maxValue    : 8000,
        valueFormat:
        {
            "int": 4,
            dec: 0
        },
    	majorTicks  : ['0','1','2','3','4','5','6','7','8'],
    	minorTicks  : 2,
    	strokeTicks : false,
    	/*highlights  : [
    		{ from : 0,   to : 100, color : 'rgba(0,   255, 0, .15)' },
    		{ from : 100, to : 160, color : 'rgba(255, 255, 0, .15)' },
    		{ from : 160, to : 220, color : 'rgba(255, 30,  0, .25)' }
    	],*/
    	highlights : false,
    	colors      :
        {
    		plate      : '#222',
    		majorTicks : '#f5f5f5',
    		minorTicks : '#ddd',
    		title      : '#fff',
    		units      : '#ccc',
    		numbers    : '#eee',
    		needle     : { start : 'rgba(240, 240, 240, 1)', end : 'rgba(255, 255, 255, 1)' }
    	}
    });

    gauge.onready = function()
    {
        // var val = 0;
    	// setInterval(function()
        // {
        //     val += 1;
        //
        //     if(val > 60) val = 0;
        //
    	// 	gauge.setValue(val);
    	// }, 100);
    };

    gauge.draw();

    gauge_rpm = gauge;
}

function knobDraw()
{

    var a = this.angle(this.cv)  // Angle
        , sa = this.startAngle          // previous start angle
        , sat = this.startAngle         // Start angle
        , ea                            // previous end angle
        , eat = sat + a                 // End angle
        , r = true;

    this.g.lineWidth = this.lineWidth;

    this.o.cursor
        && (sat = eat - 0.3)
        && (eat = eat + 0.3);

    if (this.o.displayprevious) {
        ea = this.startAngle + this.angle(this.value);
        this.o.cursor
            && (sa = ea - 0.3)
            && (ea = ea + 0.3);
        this.g.beginPath();
        this.g.strokeStyle = this.previousColor;
        this.g.arc(this.xy, this.xy, this.radius - this.lineWidth, sa, ea, false);
        this.g.stroke();
    }

    this.g.beginPath();
    this.g.strokeStyle = r ? this.o.fgColor : this.fgColor ;
    this.g.arc(this.xy, this.xy, this.radius - this.lineWidth, sat, eat, false);
    this.g.stroke();

    this.g.lineWidth = 2;
    this.g.beginPath();
    this.g.strokeStyle = this.o.fgColor;
    this.g.arc(this.xy, this.xy, this.radius - this.lineWidth + 1 + this.lineWidth * 2 / 3, 0, 2 * Math.PI, false);
    this.g.stroke();

    return false;
}

function loadDashboardHeater()
{
    $("#dashboard-heater-thermostat").knob(
    {
        min: 60,
        max: 80,
        width: 150,
        fgColor: '#fff',
        thickness: .2,
        angleOffset: 180,
        displayprevious: true,
        draw: knobDraw
    });
}

function loadDashboardAudio()
{
    $("#dashboard-audio-volume").knob(
    {
        min: 0,
        max: 100,
        width: 150,
        fgColor: '#fff',
        thickness: .2,
        angleOffset: 180,
        displayprevious: true,
        draw: knobDraw
    });
}

function loadDashboardLabels()
{
    $('#dashboard-ecu-fuelair').text('A/F: 1.34');
    $('#dashboard-ecu-oilpsi').text('OIL PSI:   12');
    $('#dashboard-ecu-coolanttemp').text('CLNT TEMP:  110');
    $('#dashboard-ecu-boostpsi').text('BOOST PSI:    9');
    $('#dashboard-ecu-danger').hide();

    $('#dashboard-heater-interiortemp').html('INT:70&deg;');
    $('#dashboard-heater-exteriortemp').html('64&deg;:EXT');
}

var socket;
function loadWebSocket()
{
    socket = new WebSocket("ws://127.0.0.1:2794", "rust-websocket");
    socket.onmessage = function(event)
    {
        var gauge = event.data.substring(0, 3);
        var val = parseInt(event.data.substring(4), 10);

    	if(gauge == 'mph')
        {
            gauge_mph.setValue(val);
        }else
        if(gauge == 'rpm')
        {
            gauge_rpm.setValue(val);
        }
    };

    setInterval(function()
    {
        socket.send('mph');
        socket.send('rpm');
    }, 100);
}
