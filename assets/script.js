(function() {
    var hash = location.pathname.split('/')[2];

    marked.setOptions({
        renderer: new marked.Renderer(),
        breaks: true,
        sanitize: true,
    });

    var render = function(text) {
        return marked.parse(text);
    };

    var renderToDom = function(text, target) {
        target.innerHTML = render(text);
    };

    var publish = function(text, cb) {
        return fetch('/publish', {
            method: 'POST',
            body: JSON.stringify({
                text: text
            })
        }).then(resp => resp.json());
    };

    var get = function(hash) {
        return fetch('/ipfs/' + hash)
            .then(resp => resp.text());
    };

    (function(enable) {
        if(!enable)return;

        var showPreview = function(really) {
            renderToDom(essayText.value, previewSection);
            editSection.hidden = really;
            previewSection.hidden = !really;
        };

        var essayText = document.getElementById('essay-text');
        var editSection = document.getElementById('edit-section');
        var previewSection = document.getElementById('preview-section');

        var form = document.getElementById('essay');
        form.onsubmit = function() {
            console.log('submit');
            var content = essayText.value;
            console.log(content);

            publish(content)
                .then(json => {
                    console.log(json);
                    location.href = '/e/' + json.hash;
                });

            return false;
        };

        var viewEdit = document.getElementById('view-edit');
        viewEdit.onclick = function() {
            showPreview(false);
            return false;
        };

        var viewPreview = document.getElementById('view-preview');
        viewPreview.onclick = function() {
            showPreview(true);
            return false;
        };
    })(!hash);

    (function(hash) {
        if(!hash)return;
        var contentSection = document.getElementById('content');

        get(hash)
            .then(text => {
                renderToDom(text, contentSection);

                var headlines = document.getElementsByTagName('h1');
                if(headlines.length) {
                    var title = headlines[0].textContent;
                    document.title = title + ' - ' + document.title;
                }
            });
    })(hash);
})();
