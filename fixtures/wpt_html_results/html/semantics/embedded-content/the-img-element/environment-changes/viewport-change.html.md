# html/semantics/embedded-content/the-img-element/environment-changes/viewport-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/environment-changes/viewport-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img viewport change</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<style>
.narrow { width:50px }
.wide { width:1000px }
</style>
<div id=log></div>
<script>
setup({explicit_done:true});

function resolve(url) {
  if (url === "") {
    return url;
  }
  var a = document.createElement('a');
  a.href = url;
  return a.href;
}

function insertIframe(className) {
  var iframe = document.createElement('iframe');
  iframe.className = className;
  iframe.src = 'iframed.sub.html?id=' + token();
  document.body.appendChild(iframe);
}
insertIframe('narrow');
insertIframe('wide');

var start_date = new Date();

onload = function() {
  var load_time = new Date() - start_date;
  var iframes = document.getElementsByTagName('iframe');
  [].forEach.call(iframes, function(iframe) {
    [].forEach.call(iframe.contentDocument.images, function(img) {
      var expected = {wide:resolve(img.dataset.wide), narrow:resolve(img.dataset.narrow)};
      var current = iframe.className;
      var next = current === 'wide' ? 'narrow' : 'wide';
      var expect_change = expected[next].indexOf('broken.png') === -1 && !('noChange' in img.dataset);

      test(function() {
        assert_equals(img.currentSrc, expected[current]);
      }, img.dataset.desc + ', onload, ' + current);

      async_test(function() {
        img.onload = this.unreached_func('Got unexpected load event');
        img.onerror = this.unreached_func('Got unexpected error event');
        if (expect_change) {
          img.onload = this.step_func_done(function() {
            assert_equals(img.currentSrc, expected[next]);
          });
        } else {
          setTimeout(this.step_func_done(), 500 + load_time);
        }
      }, img.dataset.desc + ', resize to ' + next);
    });
    iframe.classList.toggle('wide');
    iframe.classList.toggle('narrow');
  });
  done();
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/environment-changes/viewport-change.html"
}
```
