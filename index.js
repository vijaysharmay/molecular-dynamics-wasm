const lib = import("./pkg");

(() => {
    lib.then(instance => {
        const { render, init_panic_hook, mouse_move, get_clicked_node } = instance;
        const canvas = document.getElementById("canvas");
        const context = canvas.getContext("2d");

        window.addEventListener('resize', resizeCanvas, false);

        canvas.addEventListener('mousedown', function(event){
            window.sessionStorage.setItem("clickedElement", get_clicked_node({ x: parseFloat(event.x), y: parseFloat(event.y) }, getData()));
        });

        canvas.addEventListener('mousemove', function(event){
            const clickedElement = window.sessionStorage.getItem("clickedElement");
            if(clickedElement && clickedElement !== 'Node not found'){
                window.sessionStorage.setItem('data', JSON.stringify(mouse_move({ x: parseFloat(event.x), y: parseFloat(event.y) }, getData(), clickedElement)));
            }
        });

        canvas.addEventListener('mouseup', function(event){
            window.sessionStorage.removeItem('clickedElement');
        });



        function getData(){
            const initialData = {
                nodes: [
                    {id: "A", size: 5.0, position: { x: 50.0, y: 50.0 }},
                    {id: "B", size: 5.0, position: { x: 100.0, y: 50.0 }},
                    {id: "C", size: 5.0, position: { x: 100.0, y: 100.0 }},
                    {id: "D", size: 5.0, position: { x: 50.0, y: 100.0 }}
                ],
                links: [
                    {source: "A", target: "B", value: 10.0},
                    {source: "B", target: "C", value: 10.0},
                    {source: "C", target: "D", value: 10.0},
                    {source: "D", target: "A", value: 10.0}
                ]
            };

            const data = sessionStorage.getItem('data');

            if(data){
                return JSON.parse(data);
            }else{
                window.sessionStorage.setItem('data', JSON.stringify(initialData));
                return initialData;
            }
        }

        function resizeCanvas() {
            window.sessionStorage.clear();
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            init_panic_hook();
            render(getData());
        }

        resizeCanvas();
    });
})();
