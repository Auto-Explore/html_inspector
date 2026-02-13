# html/semantics/the-button-element/command-and-commandfor/on-dialog-behavior.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-dialog-behavior.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<meta name="timeout" content="long">
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/invoker-utils.js"></script>

<dialog id="invokee">
  <button id="containedinvoker" commandfor="invokee" command="close"></button>
</dialog>
<button id="invokerbutton" commandfor="invokee" command="show-modal"></button>

<script>
  function resetState() {
    invokee.close();
    try { invokee.hidePopover(); } catch {}
    invokee.removeAttribute("popover");
    invokee.returnValue = '';
    invokerbutton.setAttribute("command", "show-modal");
    containedinvoker.setAttribute("command", "close");
    containedinvoker.removeAttribute("value");
  }

  // opening a dialog

  ["show-modal", /* test case sensitivity */ "sHoW-mOdAl"].forEach(
    (command) => {
      ["property", "attribute"].forEach((setType) => {
        test(
          function (t) {
            t.add_cleanup(resetState);
            assert_false(invokee.open, "invokee.open");
            assert_false(invokee.matches(":modal"), "invokee :modal");
            if (setType === "property") {
              invokerbutton.command = command;
            } else {
              invokerbutton.setAttribute("command", command);
            }
            invokerbutton.click();
            assert_true(invokee.open, "invokee.open");
            assert_true(invokee.matches(":modal"), "invokee :modal");
          },
          `invoking (with command ${setType} as ${command}) closed dialog opens as modal`,
        );

        test(
          function (t) {
            t.add_cleanup(resetState);
            assert_false(invokee.open, "invokee.open");
            assert_false(invokee.matches(":modal"), "invokee :modal");
            invokee.addEventListener("command", (e) => e.preventDefault(), {
              once: true,
            });
            if (setType === "property") {
              invokerbutton.command = command;
            } else {
              invokerbutton.setAttribute("command", command);
            }
            invokerbutton.click();
            assert_false(invokee.open, "invokee.open");
            assert_false(invokee.matches(":modal"), "invokee :modal");
          },
          `invoking (with command ${setType} as ${command}) closed dialog with preventDefault is noop`,
        );

        test(
          function (t) {
            t.add_cleanup(resetState);
            assert_false(invokee.open, "invokee.open");
            assert_false(invokee.matches(":modal"), "invokee :modal");
            invokee.addEventListener(
              "command",
              (e) => {
                invokerbutton.setAttribute("command", "close");
              },
              { once: true },
            );
            if (setType === "property") {
              invokerbutton.command = command;
            } else {
              invokerbutton.setAttribute("command", command);
            }
            invokerbutton.click();
            assert_true(invokee.open, "invokee.open");
            assert_true(invokee.matches(":modal"), "invokee :modal");
          },
          `invoking (with command ${setType} as ${command}) while changing command still opens as modal`,
        );
      });
    },
  );

  // closing an already open dialog

  ["close", /* test case sensitivity */ "cLoSe"].forEach((command) => {
    ["property", "attribute"].forEach((setType) => {
      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          containedinvoker.click();
          assert_equals(invokee.returnValue, "");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog closes`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          containedinvoker.setAttribute("value", "foo");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "foo");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog closes and sets returnValue`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.returnValue = "test";
          containedinvoker.setAttribute("value", "foo");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "foo");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog closes and overrides returnValue`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.returnValue = "test";
          containedinvoker.setAttribute("value", "");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog closes and overrides returnValue when empty string`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.returnValue = "test";
          containedinvoker.removeAttribute("value");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "test");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog closes and doesn't override returnValue when missing value attribute`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (typeof command === "string") {
            if (setType === "property") {
              containedinvoker.command = command;
            } else {
              containedinvoker.setAttribute("command", command);
            }
          }
          invokee.addEventListener("command", (e) => e.preventDefault(), {
            once: true,
          });
          containedinvoker.click();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog with preventDefault is no-op`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.showModal();
          assert_true(invokee.open, "invokee.open");
          assert_true(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.addEventListener("command", (e) => e.preventDefault(), {
            once: true,
          });
          containedinvoker.click();
          assert_true(invokee.open, "invokee.open");
          assert_true(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open modal dialog with preventDefault is no-op`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.addEventListener(
            "command",
            (e) => {
              containedinvoker.setAttribute("command", "show");
            },
            { once: true },
          );
          containedinvoker.click();
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open dialog while changing command still closes`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.showModal();
          assert_true(invokee.open, "invokee.open");
          assert_true(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.addEventListener(
            "command",
            (e) => {
              containedinvoker.setAttribute("command", "show");
            },
            { once: true },
          );
          containedinvoker.click();
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to close (with command ${setType} as ${command}) open modal dialog while changing command still closes`,
      );
    });
  });

  // request to close an already open dialog

  ["request-close", /* test case sensitivity */ "reQuEst-Close"].forEach((command) => {
    ["property", "attribute"].forEach((setType) => {
      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          containedinvoker.click();
          assert_equals(invokee.returnValue, "");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to request-close (with command ${setType} as ${command}) open dialog closes`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          containedinvoker.setAttribute("value", "foo");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "foo");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to request-close with value (with command ${setType} as ${command}) open dialog closes and sets returnValue`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.returnValue = "test";
          containedinvoker.setAttribute("value", "foo");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "foo");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to request-close with value (with command ${setType} as ${command}) open dialog closes and overrides returnValue`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.returnValue = "test";
          containedinvoker.setAttribute("value", "");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to request-close with value (with command ${setType} as ${command}) open dialog closes and overrides returnValue when empty string`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (setType === "property") {
            containedinvoker.command = command;
          } else {
            containedinvoker.setAttribute("command", command);
          }
          invokee.returnValue = "test";
          containedinvoker.removeAttribute("value");
          containedinvoker.click();
          assert_equals(invokee.returnValue, "test");
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking to request-close with value (with command ${setType} as ${command}) open dialog closes and doesn't override returnValue when missing value attribute`,
      );

      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.show();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          if (typeof command === "string") {
            if (setType === "property") {
              containedinvoker.command = command;
            } else {
              containedinvoker.setAttribute("command", command);
            }
          }
          invokee.addEventListener("command", (e) => e.preventDefault(), {
            once: true,
          });
          containedinvoker.click();
          assert_true(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking 
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-dialog-behavior.html"
}
```
