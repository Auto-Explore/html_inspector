# html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-fragment-scrolling-samedoc.html

Counts:
- errors: 5
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-fragment-scrolling-samedoc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
  body {
    height: 2000px;
    width: 2000px;
  }

  #fragment {
    position: absolute;
    top: 800px;
    background-color: #faa;
    display: block;
    height: 100px;
    width: 100px;
  }
</style>

<body>
  <a id="fragment" name="fragment" class='box'></a>
</body>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="text/javascript">
  'use strict';

  async_test(function(t) {
    history.scrollRestoration = 'manual';
    assert_equals(history.scrollRestoration, 'manual');

    location.hash = '#fragment';
    assert_equals(window.scrollY, 800, 'new navigations should scroll to fragment');

    // create a new entry and reset the scroll before verification
    history.pushState(null, null, '#done');
    window.scrollTo(0, 0);
    assert_equals(window.scrollY, 0, 'should reset scroll before verification');

    setTimeout(function() {
      // setup verification
      window.addEventListener('hashchange', t.step_func(function() {
        assert_equals(location.hash, '#fragment');
        assert_equals(history.scrollRestoration, 'manual');
        // navigating back should give precedent to history restoration which is 'manual'
        assert_equals(window.scrollX, 0, 'should not scroll to fragment');
        assert_equals(window.scrollY, 0, 'should not scroll to fragment');
        t.done();
      }));
      // kick off verification
      window.history.back();
    }, 0);

  }, 'Manual scroll restoration should take precedent over scrolling to fragment in cross doc navigation');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 355,
        "byte_start": 310,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 409,
        "byte_start": 369,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 418,
        "byte_start": 409,
        "col": 41,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 474,
        "byte_start": 465,
        "col": 47,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 506,
        "byte_start": 475,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1674,
        "byte_start": 506,
        "col": 32,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1683,
        "byte_start": 1674,
        "col": 1,
        "line": 55
      }
    },
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-fragment-scrolling-samedoc.html"
}
```
