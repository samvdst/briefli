#let ch-letter(
  sender: (
    name: none,
    address: none,
    extra: none,
  ),
  recipient: none,
  location: none,
  date: none,
  subject: none,
  footer: none,
  font: "Arial",
  body,
) = {
  // Swiss letter standard measurements
  // Recipient address window: 22mm from left, 60mm from top, 85.5mm x 25.5mm
  let address-left = 22mm
  let address-top = 60mm
  let address-width = 85.5mm
  let address-height = 45mm // Extended to fit 5-6 lines

  // Margins: address-left determines left margin for alignment
  let margin-left = address-left
  let margin-right = 20mm
  let margin-top = 20mm
  let margin-bottom = 20mm

  // Metadata
  if sender.name != none {
    set document(title: subject, author: sender.name)
  } else {
    set document(title: subject)
  }

  // Page setup
  set page(
    paper: "a4",
    margin: (
      left: margin-left,
      right: margin-right,
      top: margin-top,
      bottom: margin-bottom,
    ),
    footer-descent: 0%,
    footer: {
      if footer != none {
        set text(size: 8pt)
        align(center, footer)
      }
    },
  )

  // Global text style
  set text(
    font: font,
    size: 11pt,
    hyphenate: false,
  )

  // Sender as return address line (small, underlined, above address window)
  // Positioned at ~45mm from top (above the 60mm address window)
  if sender.name != none or sender.address != none or sender.at("extra", default: none) != none {
    place(
      top + left,
      dy: 45mm - margin-top,
      {
        set text(size: 8pt)
        let sender-parts = ()
        if sender.name != none {
          sender-parts.push(sender.name)
        }
        if sender.address != none {
          sender-parts.push(sender.address)
        }
        if sender.at("extra", default: none) != none {
          sender-parts.push(sender.extra)
        }
        underline(sender-parts.join(" · "))
      },
    )
  }

  // Recipient block - absolutely positioned at 60mm from top
  // This ensures the address appears in the envelope window
  if recipient != none {
    place(
      top + left,
      dy: address-top - margin-top,
      box(
        width: address-width,
        {
          set text(size: 11pt)
          if type(recipient) == str {
            recipient.split(", ").join(linebreak())
          } else {
            recipient
          }
        },
      ),
    )
  }

  // Content starts after the address window area
  // Address window ends at ~105mm (60mm top + 45mm height)
  v(105mm - margin-top)

  // Place + date (left aligned, Swiss standard)
  if location != none or date != none {
    if location != none {
      location
      if date != none { ", " }
    }
    if date != none { date }
    v(8mm)
  }

  // Subject
  if subject != none {
    strong(subject)
    v(8mm)
  }

  // Body
  set par(justify: true)
  body
}
