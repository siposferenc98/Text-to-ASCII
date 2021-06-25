function feltolt()
{   
    const L = 4;
    const H = 5;
    var betuk = [];
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function()
    {
        if(this.readyState == 4 && this.status == 200)
        {
            var vissza = xhttp.responseText;
            var feldarabol = vissza.split("\n");
            for (let i = 0; i < H; i++)
            {
                betuk[i] = [];
                var k = 0;
                for (let j = 0; j < feldarabol[i].length; j+=L)
                {
                    var substring = feldarabol[i].substr(j,L);
                    betuk[i][k] = substring;
                    k++;
                }
            }
            
            var abc = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',' ','?']
            var bemenet = document.getElementById("szoveg").value.toUpperCase();
            var indexek = [];
            var nincs = 0;

            for (let i = 0; i < bemenet.length; i++)
            {
                for (let j = 0; j < abc.length; j++)
                {
                    nincs++;
                    if (abc[j] == bemenet[i])
                    {
                        indexek[i] = j;
                        nincs = 0;
                    }
                    else if(nincs == abc.length)
                    {
                        indexek[i] = 27;
                    }
                }
                nincs = 0;
            }

            var ki = document.getElementById("ki");
            ki.value = "";
            var answer = [];
            for (let i = 0; i < H; i++)
            {

                answer[i] = [];
                for (let j = 0; j < indexek.length; j++)
                {
                    answer[i][j] = betuk[i][indexek[j]];
                }
                var x = answer[i].join('')+"\n";
                /*var x1 = x.replace(/ /g, "&nbsp;");*/
                ki.value = ki.value + x;
            }
            
        }
    };
    xhttp.open("GET", "alphabet.txt", true);
    xhttp.send();
    
    
    
    
}

/*for (let i = 0; i < H; i++) {
    const ROW = readline();
    betuk[i] = [];
    var k = 0;
    for (let j = 0; j < ROW.length; j+=L)
    {
        var substring = ROW.substr(j,L);
        betuk[i][k] = substring;
        k++;
    }
}*/

