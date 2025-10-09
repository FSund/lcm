add_executable(lcm-sink lcm-sink.c)
target_link_libraries(lcm-sink lcm-static)

add_executable(lcm-source lcm-source.c)
target_link_libraries(lcm-source lcm-static ${lcm-winport})

add_executable(lcm-tester lcm-tester.c)
# Use GLib2 target (works better with vcpkg and Windows)
target_link_libraries(lcm-tester PRIVATE lcm-static GLib2::glib)

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-tester PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

add_executable(lcm-example lcm-example.c)
target_link_libraries(lcm-example lcm-static)
if(WIN32)
  target_link_libraries(lcm-example ws2_32)
endif()

add_executable(lcm-logfilter lcm-logfilter.c)
# Use GLib2 target (works better with vcpkg and Windows)
target_link_libraries(lcm-logfilter PRIVATE lcm-static ${lcm-winport} GLib2::glib)

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-logfilter PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

add_executable(lcm-buftest-receiver buftest-receiver.c)
# Use GLib2 target (works better with vcpkg and Windows)
target_link_libraries(lcm-buftest-receiver PRIVATE lcm-static GLib2::glib)

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-buftest-receiver PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

add_executable(lcm-buftest-sender buftest-sender.c)
# Use GLib2 target (works better with vcpkg and Windows)
target_link_libraries(lcm-buftest-sender PRIVATE lcm-static GLib2::glib)

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-buftest-sender PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

install(TARGETS
  lcm-sink
  lcm-source
  lcm-tester
  lcm-example
  lcm-logfilter
  DESTINATION bin
)

install(FILES
  lcm-example.1
  lcm-sink.1
  lcm-source.1
  lcm-tester.1
  DESTINATION share/man/man1
)
