# html/semantics/embedded-content/the-img-element/source-media-outside-doc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/source-media-outside-doc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Image source selection using media queries is performed for img elements outside the document</title>
<link rel="help" href="https://html.spec.whatwg.org/#reacting-to-environment-changes">
<link rel="help" href="https://html.spec.whatwg.org/#reacting-to-dom-mutations">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe width="350" height="100" onload="async_test(this.contentWindow.run)" srcdoc="
<!DOCTYPE html>
<script>
const { assert_equals } = parent;
const iframe = parent.document.querySelector('iframe');

function run(t) {
  const picture = document.createElement('picture');

  const source1 = document.createElement('source');
  source1.setAttribute('media', '(min-width: 300px)');
  source1.setAttribute('srcset', 'data:,a');
  picture.append(source1);

  const source2 = document.createElement('source');
  source2.setAttribute('media', '(min-width: 200px)');
  source2.setAttribute('srcset', 'data:,b');
  picture.append(source2);

  const img = document.createElement('img');
  img.src = 'data:,c';
  picture.append(img);

  queueMicrotask(t.step_func(function() {
    assert_equals(img.currentSrc, 'data:,a', 'Initial currentSrc value');
    matchMedia(source1.media).addEventListener(
      'change',
      function() {
        queueMicrotask(t.step_func(function() {
          assert_equals(img.currentSrc, 'data:,b', 'After MQ change');
          img.remove();
          queueMicrotask(t.step_func(function() {
            assert_equals(img.currentSrc, 'data:,c', 'After removing img');
            t.done();
          }));
        }));
      },
      { once: true }
    );
    iframe.width = 250;
  }));
}
</script>
"></iframe>
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
  "source_name": "html/semantics/embedded-content/the-img-element/source-media-outside-doc.html"
}
```
