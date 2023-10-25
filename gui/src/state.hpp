#include <string>
#include <vector>
#include "math.hpp"

enum VolumeType {
    RAW,
    BVP
};

class VolumeInfo {
    private:
        std::string filePath;
        VolumeType type;
        Vector3Uint dimensions;
};

////////////////

class Operation {
    public:
        bool operatesOnVolume(VolumeInfo* volumeInfo);
};

////////////////

class State {
    public:
        State();
        void addVolume(VolumeInfo* volumeInfo);
        void addOperation(Operation* operation);
        void removeVolumeInfo(VolumeInfo* volumeInfo);
        void removeOperation(Operation* operation);

    private:
        std::vector<VolumeInfo*> volumes;
        std::vector<Operation*> operations;
};

inline State::State() {
    this->volumes = {};
    this->operations = {};
}

inline void State::addVolume(VolumeInfo* volumeInfo) {
    this->volumes.push_back(volumeInfo);
}

inline void State::addOperation(Operation* operation) {
    this->operations.push_back(operation);
}

inline void State::removeVolumeInfo(VolumeInfo* volumeInfo) {
    // We first remove any operations that operate on the volume
    // to avoid invalid volume indices
    for (int i = 0; i < this->operations.size(); i++) {
        if (this->operations[i]->operatesOnVolume(volumeInfo)) {
            this->operations.erase(this->operations.begin() + i);
        }
    }

    for (int i = 0; i < this->volumes.size(); i++) {
        if (volumeInfo == this->volumes[i]) {
            this->volumes.erase(this->volumes.begin() + i);
            break;
        }
    }

    delete volumeInfo;
}

inline void State::removeOperation(Operation* operation) {
    for (int i = 0; i < this->volumes.size(); i++) {
        if (this->operations[i] == operation) {
            this->operations.erase(this->operations.begin() + i);
        }
    }

    delete operation;
}