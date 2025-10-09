add_executable(lcm-logger lcm_logger.c glib_util.c)
# Use GLib2 target (works better with vcpkg and Windows)
target_link_libraries(lcm-logger PRIVATE lcm-static ${lcm-winport} GLib2::glib)

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-logger PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

add_executable(lcm-logplayer lcm_logplayer.c)
# Use GLib2 target (works better with vcpkg and Windows)
target_link_libraries(lcm-logplayer PRIVATE lcm-static ${lcm-winport} GLib2::glib)

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-logplayer PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

install(TARGETS
  lcm-logger
  lcm-logplayer
  DESTINATION bin
)

install(FILES
  lcm-logger.1
  lcm-logplayer.1
  DESTINATION share/man/man1
)
