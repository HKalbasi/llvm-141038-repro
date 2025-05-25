#include <cstdint>
#include <array>

extern "C" {
    void push_to_vec(void *v, uint64_t i);
    void new_vec_in_stack(void *v);
    void free_vec_in_stack(void *v);
}

struct MyVec {
    alignas(8) std::array<uint8_t, 24> data;

    MyVec() {
        new_vec_in_stack(reinterpret_cast<void*>(data.begin()));
    }

    // ~MyVec() {
    //     free_vec_in_stack(reinterpret_cast<void*>(data.begin()));
    // }
};

// struct Darivari {
//     ~Darivari() {

//     }
// }

void build_vec(int n)
{
    // Darivari d{};
    MyVec v;
    void* vec = reinterpret_cast<void*>(v.data.begin());

    for (int i = 0; i < n; i++)
    {
        push_to_vec(vec, i);
    }

    free_vec_in_stack(vec);
}


extern "C" {
    void do_the_job()
    {
        for (int i = 0; i < 100000; i++)
        {
            build_vec(10000);
        }
    }
}
