# html/browsers/the-window-object/open-close/open-features-tokenization-screenx-screeny.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/open-features-tokenization-screenx-screeny.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML: window.open `features`: tokenization -- legacy position features `screenx`, `screeny`</title>
<meta name=timeout content=long>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#apis-for-creating-and-navigating-browsing-contexts-by-name">

<!-- user agents are not required to support open features other than `noopener`
     and on some platforms position and size features don't make sense -->
<meta name="flags" content="may" />

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/PrefixedPostMessage.js"></script>
<script>
var windowURL = 'resources/message-opener.html';
var width = 'width=401,';
var height = 'height=402,';

[ 'screenx=141',
  ' screenx = 141',
  'screenx==141',
  '\nscreenx= 141',
  ',screenx=141,,',
  'SCREENX=141',
  'screenX=141'
].forEach((features, idx, arr) => {
  async_test(t => {
    var prefixedMessage = new PrefixedMessageTest();
    prefixedMessage.onMessage(t.step_func_done((data, e) => {
      e.source.close();
      assert_equals(data.left, 141);
    }));
    var win = window.open(prefixedMessage.url(windowURL) + '&expected_screenX=141', '', width + height + features);
  }, `${format_value(features)} should set left position of opened window`);
});

[ 'screeny=142',
  ' screeny = 142',
  'screeny==142',
  '\nscreeny= 142',
  ',screeny=142,,',
  'SCREENY=142',
  'screenY=142'
].forEach((features, idx, arr) => {
  async_test(t => {
    var prefixedMessage = new PrefixedMessageTest();
    prefixedMessage.onMessage(t.step_func_done((data, e) => {
      e.source.close();
      assert_equals(data.top, 142);
    }));
    var win = window.open(prefixedMessage.url(windowURL) + '&expected_screenY=142', '', width + height + features);
  }, `${format_value(features)} should set top position of opened window`);
});

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
  "source_name": "html/browsers/the-window-object/open-close/open-features-tokenization-screenx-screeny.html"
}
```
