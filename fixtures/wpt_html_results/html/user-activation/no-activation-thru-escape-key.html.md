# html/user-activation/no-activation-thru-escape-key.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/no-activation-thru-escape-key.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>No user activation through 'Escape' key</title>
    <meta name="timeout" content="long">
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
    <link rel="author" title="Google" href="http://www.google.com "/>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#tracking-user-activation">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/resources/testdriver.js"></script>
    <script src="/resources/testdriver-vendor.js"></script>
    <style>
      #target {
        width: 40ex;
        background-color: yellow;
      }
    </style>
    <script type="text/javascript">
      let keydown_event_fired = false;
      let keyup_event_fired = false;

      function run() {
        let textbox_elem = document.getElementById("target");
        let test_esc_key = async_test("'Escape' key doesn't activate a page.");

        test_esc_key.step(() => {
          assert_true(!!navigator.userActivation, "This test requires user activation query API");
        });

        textbox_elem.focus();

        on_event(textbox_elem, "keydown", () => {
          test_esc_key.step(() => {
            keydown_event_fired = true;
            assert_false(navigator.userActivation.isActive, "No user activation on keydown");
          });
        });

        on_event(textbox_elem, "keyup", () => {
          test_esc_key.step(() => {
            if (keydown_event_fired)
              keyup_event_fired = true;
            assert_true(keydown_event_fired, "keydown event fired before keyup");
            assert_false(navigator.userActivation.isActive, "No user activation on keyup");
          });
        });

        // Inject mouse inputs.
        const escape_key = "\uE00C";
        test_driver
        .send_keys(textbox_elem, escape_key)
        .then(() => {
          assert_true(keyup_event_fired, "keydown event fired before keyup");
          test_esc_key.done();
        });
      }
    </script>
  </head>
  <body onload="run()">
    <h1>No user activation through 'Escape' key</h1>
    <h4>Tests that pressing/releasing 'Escape' key is not treated as a user activation.</h4>
    <input id="target" value="Press and release the 'Esc' key." />
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “http://www.google.com ” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 274,
        "byte_start": 209,
        "col": 5,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 751,
        "byte_start": 720,
        "col": 5,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h4” (with computed level 4) follows the heading “h1” (with computed level 1), skipping 2 heading levels.",
      "severity": "Warning",
      "span": {
        "byte_end": 2175,
        "byte_start": 2171,
        "col": 5,
        "line": 62
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/user-activation/no-activation-thru-escape-key.html"
}
```
