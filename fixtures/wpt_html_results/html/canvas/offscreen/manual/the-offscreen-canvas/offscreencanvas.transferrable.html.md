# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transferrable.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transferrable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/#offscreencanvas">

<script id="myWorker" type="text/worker">

function test1(offscreenCanvas)
{
    return offscreenCanvas.width == 10 && offscreenCanvas.height == 10;
}

self.onmessage = function(e) {
    switch(e.data.msg) {
        case 'test1':
            self.postMessage(test1(e.data.data));
            break;
    }
};

</script>

<script>
function makeWorker(script)
{
    var blob = new Blob([script]);
    return new Worker(URL.createObjectURL(blob));
}

async_test(function(t) {
    var worker = makeWorker(document.getElementById("myWorker").textContent);
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    worker.postMessage({msg: 'test1', data: offscreenCanvas}, [offscreenCanvas]);
    worker.addEventListener('message', t.step_func_done(function(msg) {
        assert_true(msg.data);
    }));
    assert_equals(offscreenCanvas.width, 0);
    assert_equals(offscreenCanvas.height, 0);
}, "Test that offscreenCanvas's size is correct after being transferred to a worker.");


test(function() {
    function testException(contextType) {
        var worker = makeWorker(document.getElementById("myWorker").textContent);
        var offscreenCanvas = new OffscreenCanvas(10, 10);
        var ctx = offscreenCanvas.getContext(contextType);
        assert_throws_dom("InvalidStateError", function() {
            worker.postMessage({offscreenCanvas}, [offscreenCanvas]);
        });
    }
    testException('2d');
}, "Test that transfer an OffscreenCanvas that already have a 2d context throws exception.");

test(function() {
    var worker = makeWorker(document.getElementById("myWorker").textContent);
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    worker.postMessage({offscreenCanvas}, [offscreenCanvas]);
    assert_throws_dom("DataCloneError", function() {
        worker.postMessage({offscreenCanvas}, [offscreenCanvas]);
    });
}, "Test that transfer an OffscreenCanvas twice throws exception.");

test(function() {
    var worker = makeWorker(document.getElementById("myWorker").textContent);
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    worker.postMessage({offscreenCanvas}, [offscreenCanvas]);
    assert_throws_dom("InvalidStateError", function() {
        offscreenCanvas.getContext('2d');
    });
}, "Test that calling getContext('2d') on a detached OffscreenCanvas throws exception.");

test(function() {
    var worker = makeWorker(document.getElementById("myWorker").textContent);
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    worker.postMessage({offscreenCanvas}, [offscreenCanvas]);
    assert_throws_dom("InvalidStateError", function() {
        offscreenCanvas.getContext('webgl');
    });
}, "Test that calling getContext('webgl') on a detached OffscreenCanvas throws exception.");

</script>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transferrable.html"
}
```
