#!/usr/bin/env emacs --script

(setq debug-on-error t)

(setq pos 0)
(setq depth 0)
(setq aim 0)

(defun filename-to-list (filename)
  (split-string
   (with-temp-buffer
     (insert-file-contents filename)
     (buffer-string))
   "\n" t)
  )

(dolist (line (let ((filename (expand-file-name (car (last command-line-args)))))
                (filename-to-list filename)
                ))
  (let ((instruction (split-string line)))
    (let ((action (car instruction))
          (value (string-to-number (car (cdr instruction))))
          )
      (pcase action
        ("forward"
         (setq pos (+ pos value))
         (setq depth (+ depth (* aim value)))
         )
        ("down" (setq aim (+ aim value)))
        ("up" (setq aim (- aim value)))
        )
      )
    )
  )

(princ (* pos depth))
(princ "\n")
