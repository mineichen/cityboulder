var data = 0;
fetch("/data/bu")
    .then(function(raw) { return raw.text(); })
    .then(function(data) {
        console.log(data);
        //var data = [[new Date("2020-01-01"), 0], [new Date("2020-01-02"), 1]];
        var g = new Dygraph(
            document.getElementById("graphdiv"),  // containing div
            data, {
                title: "Besucher im Cityboulder",
                drawPoints: true,
                //showRoller: true,
                labels: ['Time', 'Besucher']
            }
        );
    });
