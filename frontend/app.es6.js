import 'whatwg-fetch'

window.addEventListener('load', function() {
    var marked = require('marked');

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
            headers: new Headers({
                'Content-Type': 'application/json'
            }),
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

            viewEdit.disabled = !really;
            viewPreview.disabled = really;
        };

        var essayText = document.getElementById('essay-text');
        var editSection = document.getElementById('edit-section');
        var previewSection = document.getElementById('preview-section');
        var submitButton = document.getElementById('submit-button');

        var viewEdit = document.getElementById('view-edit');
        var viewPreview = document.getElementById('view-preview');

        var form = document.getElementById('essay');
        form.onsubmit = function(e) {
            e.preventDefault();
            console.log('submit');
            var content = essayText.value;
            console.log(content);

            submitButton.disabled = true;
            submitButton.value = 'submitting...';

            publish(content)
                .then(json => {
                    submitButton.value = 'publish';
                    submitButton.disabled = false;
                    console.log(json);
                    if(json.hash) {
                        location.href = '/e/' + json.hash;
                    } else {
                        alert('failed to publish: server didn\'t return hash');
                    }
                })
                .catch(e => {
                    submitButton.value = 'publish';
                    submitButton.disabled = false;
                    console.log(e);
                    alert(e.message);
                });

            return false;
        };

        viewEdit.onclick = function() {
            showPreview(false);
            return false;
        };

        viewPreview.onclick = function() {
            showPreview(true);
            return false;
        };
    })(!hash);

    (function(hash) {
        if(!hash) return;
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
});
