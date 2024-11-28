export class histories {
    constructor(){
        this.histories = []
    }

    print_history() {
        const historyContainerId = document.getElementById("history_container");
        
        for (let index = 0; 0 < historyContainerId.childElementCount; index++) {
            historyContainerId.removeChild(historyContainerId.firstElementChild);
        }

        for (let i = 0; i < this.histories.length; i++) {
            let label = this.histories[i].label;
            let fen = this.histories[i].fen;
            console.log(label.concat(fen));
        
            let node = document.createElement("span");
            let textnode = document.createTextNode(label.concat(" "));

            this.histories[i].id = i;
        
            node.appendChild(textnode);
            node.value = fen;
            node.onclick = () => this.histories[i].history_click();
        
            historyContainerId.appendChild(node);
        }
    }

    

}

var history_array = new histories();

export class history {
    constructor(label, fen) {
        this.fen = fen;
        this.label = label;
        this.id = 0;
    }

    add_to_history() {
        history_array.histories.push(this);
        console.log(history_array);
        history_array.print_history()
    }

    history_click() {
        console.log(this.id);
    }

}



